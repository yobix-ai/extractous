package ai.yobix;

import java.io.*;
import java.util.concurrent.Executor;

import org.apache.tika.parser.ParseContext;
import org.apache.tika.parser.Parser;
import org.xml.sax.ContentHandler;
import org.apache.tika.exception.ZeroByteFileException;
import org.apache.tika.metadata.Metadata;
import org.apache.tika.metadata.TikaCoreProperties;
import org.apache.tika.sax.BodyContentHandler;
import org.apache.tika.sax.ToXMLContentHandler;

public class ParsingReader extends Reader {

    private final Parser parser;
    private final Reader reader;
    private final PipedOutputStream pipedOutputStream;
    private final InputStream stream;
    private final Metadata metadata;
    private final ParseContext context;
    private final boolean outputXml;
    private final String encoding;
    private transient Throwable throwable;

    public ParsingReader(Parser parser, InputStream stream, Metadata metadata,
                            ParseContext context, boolean outputXml, String encoding) throws IOException {
        this.parser = parser;
        this.stream = stream;
        this.metadata = metadata;
        this.context = context;
        this.outputXml = outputXml;
        this.encoding = encoding;

        PipedInputStream pipedInputStream = new PipedInputStream();
        this.pipedOutputStream = new PipedOutputStream(pipedInputStream);
        this.reader = new BufferedReader(new InputStreamReader(pipedInputStream));

        Executor executor = command -> {
            String name = metadata.get(TikaCoreProperties.RESOURCE_NAME_KEY);
            if (name != null) {
                name = "Apache Tika: " + name;
            } else {
                name = "Apache Tika";
            }
            Thread thread = new Thread(command, name);
            thread.setDaemon(true);
            thread.start();
        };

        executor.execute(new ParsingTask());

        reader.mark(1);
        reader.read();
        reader.reset();
    }

    @Override
    public int read(char[] cbuf, int off, int len) throws IOException {
        if (throwable instanceof ZeroByteFileException) {
            return -1;
        } else if (throwable instanceof IOException) {
            throw (IOException) throwable;
        } else if (throwable != null) {
            throw new IOException("", throwable);
        }
        return reader.read(cbuf, off, len);
    }

    @Override
    public void close() throws IOException {
        reader.close();
    }

    private class ParsingTask implements Runnable {

        public void run() {
            try {
                ContentHandler handler = outputXml ? new ToXMLContentHandler(pipedOutputStream, encoding) : new BodyContentHandler(pipedOutputStream);
                parser.parse(stream, handler, metadata, context);
            } catch (Throwable t) {
                throwable = t;
            }

            try {
                stream.close();
            } catch (Throwable t) {
                if (throwable == null) {
                    throwable = t;
                }
            }

            try {
                pipedOutputStream.close();
            } catch (Throwable t) {
                if (throwable == null) {
                    throwable = t;
                }
            }
        }

    }
}
