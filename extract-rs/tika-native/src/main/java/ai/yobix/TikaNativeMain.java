package ai.yobix;

import org.apache.tika.Tika;
import org.apache.tika.exception.TikaException;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
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

    public static TikaResult detect(String filePath) {
        final Path path = Paths.get(filePath);
        final Metadata metadata = new Metadata();

        try (final InputStream stream = TikaInputStream.get(path, metadata)) {
            return new TikaResult(tika.detect(stream, metadata));

        } catch (java.io.IOException e) {
            return new TikaResult((byte) 1, e.getMessage());
        }
    }

    /**
     * Parses the given file and returns its content as String.
     * To avoid unpredictable excess memory use, the returned string contains only up to maxLength (parameter)
     * first characters extracted from the input document.
     *
     * @param filePath:  the path of the file to be parsed
     * @param maxLength: maximum length of the returned string
     * @return String representing the content
     */
    public static TikaResult parseToStringWithLength(String filePath, int maxLength) {
        try {
            final Path path = Paths.get(filePath);
            final Metadata metadata = new Metadata();
            final InputStream stream = TikaInputStream.get(path, metadata);

            // No need to close the stream because parseToString does so
            return new TikaResult(tika.parseToString(stream, metadata, maxLength));
        } catch (java.io.IOException e) {
            return new TikaResult((byte) 1, "Could not open file: "+ e.getMessage());
        } catch (TikaException e) {
            return new TikaResult((byte) 2, "Parse error occurred : "+ e.getMessage());
        }
    }

    /**
     * Parses the given file and returns its content as String. By default, the max string length is 100_000 chars
     *
     * @param filePath the path of the file
     * @return TikaResult
     */
    public static TikaResult parseToString(String filePath) {
        return parseToStringWithLength(filePath, tika.getMaxStringLength());
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