package ai.yobix;

import org.apache.tika.Tika;
import org.apache.tika.exception.TikaException;

import java.io.IOException;

import org.graalvm.nativeimage.IsolateThread;
import org.graalvm.nativeimage.c.function.CEntryPoint;
import org.graalvm.nativeimage.c.type.CCharPointer;
import org.graalvm.nativeimage.c.type.CConst;
import org.graalvm.nativeimage.c.type.CTypeConversion;

public class TikaNativeMain {

    private static final Tika tika = new Tika();

    public static String detect(String filePath) throws IOException {
        return tika.detect(new java.io.File(filePath));
    }

    /**
     * Loads a file and parses it into a string
     * Intended to be used by the JNI invocation interface
     * @param filePath: the path of the file to be parsed
     * @return String representing the content
     */
    public static String parseToString(String filePath) {
        try {
            return tika.parseToString(new java.io.File(filePath));
        } catch (java.io.IOException | TikaException e) {
            throw new RuntimeException(e);
        }
    }

    /**
     * This is the main entry point of the native image build. @CEntryPoint is used
     * because we do not want to build an executable with a main method. The gradle nativeImagePlugin
     * expects either a main or @CEntryPoint
     */
    @CEntryPoint(name = "c_parse_to_string")
    private static CCharPointer cParseToString(IsolateThread thread, @CConst CCharPointer cFilePath) {
        String filePath = CTypeConversion.toJavaString(cFilePath);

        String result = parseToString(filePath);
        try (CTypeConversion.CCharPointerHolder holder = CTypeConversion.toCString(result)) {
            return holder.get();
        }
    }

    // A dummy main function to be used as the entry point for the native image build
//    public static void main(String[] args) {
//        System.out.println(" -- TikaNativeMain called");
//        System.out.println(" --    Tika Max string length : " + tika.getMaxStringLength());
//
//        parseToString(args[0]);
//
//        // This is very important to generate the correct entry with the exit method
//        // for the java.lang.System class in jni-config.json
//        // We need the System.exit method to be called from c through JNI to clear all awt resources
//        // and not let sun.java2d.Disposer waiting forever
//        System.exit(0);
//    }
}