package ai.yobix;

import org.apache.commons.io.input.ReaderInputStream;
import org.apache.tika.Tika;
import org.apache.tika.exception.TikaException;

import java.io.InputStream;
import java.io.Reader;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.nio.file.Paths;

import org.apache.tika.io.TikaInputStream;
import org.apache.tika.metadata.Metadata;
import org.graalvm.nativeimage.IsolateThread;
import org.graalvm.nativeimage.c.function.CEntryPoint;
import org.graalvm.nativeimage.c.type.CCharPointer;
import org.graalvm.nativeimage.c.type.CConst;
import org.graalvm.nativeimage.c.type.CTypeConversion;

public class TikaNativeMain {

    private static final Tika tika = new Tika();

    /**
     * Parses the given file and returns its type as a mime type
     * @param filePath:  the path of the file to be parsed
     * @return StringResult
     */
    public static StringResult detect(String filePath) {
        final Path path = Paths.get(filePath);
        final Metadata metadata = new Metadata();

        try (final InputStream stream = TikaInputStream.get(path, metadata)) {
            return new StringResult(tika.detect(stream, metadata));

        } catch (java.io.IOException e) {
            return new StringResult((byte) 1, e.getMessage());
        }
    }

    /**
     * Parses the given file and returns its content as String.
     * To avoid unpredictable excess memory use, the returned string contains only up to maxLength
     * first characters extracted from the input document.
     *
     * @param filePath:  the path of the file to be parsed
     * @param maxLength: maximum length of the returned string
     * @return StringResult
     */
    public static StringResult parseToStringWithLength(String filePath, int maxLength) {
        try {
            final Path path = Paths.get(filePath);
            final Metadata metadata = new Metadata();
            final InputStream stream = TikaInputStream.get(path, metadata);

            // No need to close the stream because parseToString does so
            return new StringResult(tika.parseToString(stream, metadata, maxLength));
        } catch (java.io.IOException e) {
            return new StringResult((byte) 1, "Could not open file: "+ e.getMessage());
        } catch (TikaException e) {
            return new StringResult((byte) 2, "Parse error occurred : "+ e.getMessage());
        }
    }

    /**
     * Parses the given file and returns its content as String. By default, the max string length is 100_000 chars
     *
     * @param filePath the path of the file
     * @return StringResult
     */
    public static StringResult parseToString(String filePath) {
        return parseToStringWithLength(filePath, tika.getMaxStringLength());
    }

    /**
     * Parses the given file and returns its content as Reader. The reader can be used
     * to read chunks and must be closed when reading is finished
     *
     * @param filePath the path of the file
     * @return ReaderResult
     */
    public static ReaderResult parse(String filePath) {
        try {
            final Path path = Paths.get(filePath);
            final Reader reader = tika.parse(path);

            // Convert Reader which works with chars to ReaderInputStream which works with bytes
            ReaderInputStream readerInputStream = ReaderInputStream.builder()
                    .setReader(reader)
                    .setCharset(StandardCharsets.UTF_8)
                    .get();

            return new ReaderResult(readerInputStream);
        } catch (java.io.IOException e) {
            return new ReaderResult((byte) 1, "Could not open file: "+ e.getMessage());
        }
    }

    /**
     * This is the main entry point of the native image build. @CEntryPoint is used
     * because we do not want to build an executable with a main method. The gradle nativeImagePlugin
     * expects either a main method or @CEntryPoint
     * This uses the C Api isolate, which is can only work with primitive return types unlike the JNI invocation
     * interface.
     */
    @CEntryPoint(name = "c_parse_to_string")
    private static CCharPointer cParseToString(IsolateThread thread, @CConst CCharPointer cFilePath) {
        final String filePath = CTypeConversion.toJavaString(cFilePath);

        final Path path = Paths.get(filePath);
        try {
            final String content = tika.parseToString(path);
            try (CTypeConversion.CCharPointerHolder holder = CTypeConversion.toCString(content)) {
                return holder.get();
            }

        } catch (java.io.IOException | TikaException e) {
            throw new RuntimeException(e);
        }
    }

}