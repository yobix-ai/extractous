package ai.yobix;

import org.apache.commons.io.input.ReaderInputStream;
import org.apache.tika.parser.ParsingReader;
import org.apache.tika.Tika;
import org.apache.tika.config.TikaConfig;
import org.apache.tika.exception.TikaException;

import java.io.InputStream;
import java.io.Reader;
import java.net.MalformedURLException;
import java.net.URI;
import java.net.URISyntaxException;
import java.net.URL;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.nio.file.Paths;

import org.apache.tika.io.TikaInputStream;
import org.apache.tika.metadata.Metadata;
import org.apache.tika.parser.AutoDetectParser;
import org.apache.tika.parser.ParseContext;
import org.apache.tika.parser.Parser;
import org.apache.tika.parser.microsoft.OfficeParserConfig;
import org.apache.tika.parser.ocr.TesseractOCRConfig;
import org.apache.tika.parser.pdf.PDFParserConfig;
import org.graalvm.nativeimage.IsolateThread;
import org.graalvm.nativeimage.c.function.CEntryPoint;
import org.graalvm.nativeimage.c.type.CCharPointer;
import org.graalvm.nativeimage.c.type.CConst;
import org.graalvm.nativeimage.c.type.CTypeConversion;

public class TikaNativeMain {

    private static final Tika tika = new Tika();

    /**
     * Parses the given file and returns its type as a mime type
     *
     * @param filePath: the path of the file to be parsed
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
     * Parse tika metadata to array String.
     * @param metadata: Tika Metadata
     * @return String[]
     */
    private static String[] parseMetadata(Metadata metadata) {
        String[] result = new String[metadata.size()];
        int i = 0;
        for (String name : metadata.names()) {
            result[i] = name + ":" + metadata.get(name);
            i++;
        }
        return result;
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
    public static StringResult parseToString(String filePath, int maxLength) {
        try {
            final Path path = Paths.get(filePath);
            final Metadata metadata = new Metadata();
            final InputStream stream = TikaInputStream.get(path, metadata);
            String parseString = tika.parseToString(stream, metadata, maxLength);
            return new StringResult(parseString, parseMetadata(metadata));
        } catch (java.io.IOException e) {
            return new StringResult((byte) 1, "Could not open file: " + e.getMessage());
        } catch (TikaException e) {
            return new StringResult((byte) 2, "Parse error occurred : " + e.getMessage());
        }
    }

    /**
     * Parses the given file and returns its content as Reader. The reader can be used
     * to read chunks and must be closed when reading is finished
     *
     * @param filePath the path of the file
     * @return ReaderResult
     */
    public static ReaderResult parseFile(
            String filePath,
            String charsetName,
            PDFParserConfig pdfConfig,
            OfficeParserConfig officeConfig,
            TesseractOCRConfig tesseractConfig
    ) {
        try {
//            System.out.println("pdfConfig.isExtractInlineImages = " + pdfConfig.isExtractInlineImages());
//            System.out.println("pdfConfig.isExtractMarkedContent = " + pdfConfig.isExtractMarkedContent());
//            System.out.println("pdfConfig.getOcrStrategy = " + pdfConfig.getOcrStrategy());
//            System.out.println("officeConfig.isIncludeHeadersAndFooters = " + officeConfig.isIncludeHeadersAndFooters());
//            System.out.println("officeConfig.isIncludeShapeBasedContent = " + officeConfig.isIncludeShapeBasedContent());
//            System.out.println("ocrConfig.getTimeoutSeconds = " + tesseractConfig.getTimeoutSeconds());
//            System.out.println("ocrConfig.language = " + tesseractConfig.getLanguage());

            final Path path = Paths.get(filePath);
            final Metadata metadata = new Metadata();
            final TikaInputStream stream = TikaInputStream.get(path, metadata);

            return parse(stream, metadata, charsetName, pdfConfig, officeConfig, tesseractConfig);

        } catch (java.io.IOException e) {
            return new ReaderResult((byte) 1, "Could not open file: " + e.getMessage());
        }
    }

    /**
     * Parses the given Url and returns its content as Reader. The reader can be used
     * to read chunks and must be closed when reading is finished
     *
     * @param urlString the url to be parsed
     * @return ReaderResult
     */
    public static ReaderResult parseUrl(
            String urlString,
            String charsetName,
            PDFParserConfig pdfConfig,
            OfficeParserConfig officeConfig,
            TesseractOCRConfig tesseractConfig
    ) {
        try {
            final URL url = new URI(urlString).toURL();
            final Metadata metadata = new Metadata();
            final TikaInputStream stream = TikaInputStream.get(url, metadata);

            return parse(stream, metadata, charsetName, pdfConfig, officeConfig, tesseractConfig);

        } catch (MalformedURLException e) {
            return new ReaderResult((byte) 2, "Malformed URL error occurred " + e.getMessage());
        } catch (URISyntaxException e) {
            return new ReaderResult((byte) 3, "Malformed URI error occurred: " + e.getMessage());
        } catch (java.io.IOException e) {
            return new ReaderResult((byte) 1, "IO error occurred: " + e.getMessage());
        }
    }

    /**
     * Parses the given array of bytes and return its content as Reader. The reader can be used
     * to read chunks and must be closed when reading is finished
     *
     * @param data an array of bytes
     * @return ReaderResult
     */
    public static ReaderResult parseBytes(
            byte[] data,
            String charsetName,
            PDFParserConfig pdfConfig,
            OfficeParserConfig officeConfig,
            TesseractOCRConfig tesseractConfig
    ) {

        final Metadata metadata = new Metadata();
        final TikaInputStream stream = TikaInputStream.get(data, metadata);

        return parse(stream, metadata, charsetName, pdfConfig, officeConfig, tesseractConfig);
    }

    private static ReaderResult parse(
            TikaInputStream inputStream,
            Metadata metadata,
            String charsetName,
            PDFParserConfig pdfConfig,
            OfficeParserConfig officeConfig,
            TesseractOCRConfig tesseractConfig
    ) {
        try {

            final TikaConfig config = TikaConfig.getDefaultConfig();
            final ParseContext parsecontext = new ParseContext();
            final Parser parser = new AutoDetectParser(config);

            parsecontext.set(Parser.class, parser);
            parsecontext.set(PDFParserConfig.class, pdfConfig);
            parsecontext.set(OfficeParserConfig.class, officeConfig);
            parsecontext.set(TesseractOCRConfig.class, tesseractConfig);

            final Reader reader = new ParsingReader(parser, inputStream, metadata, parsecontext);

            // Convert Reader which works with chars to ReaderInputStream which works with bytes
            ReaderInputStream readerInputStream = ReaderInputStream.builder()
                    .setReader(reader)
                    .setCharset(Charset.forName(charsetName, StandardCharsets.UTF_8))
                    .get();

            return new ReaderResult(readerInputStream);

        } catch (java.io.IOException e) {
            return new ReaderResult((byte) 1, "IO error occurred: " + e.getMessage());
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
