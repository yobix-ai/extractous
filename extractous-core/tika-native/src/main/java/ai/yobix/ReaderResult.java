package ai.yobix;

import org.apache.commons.io.input.ReaderInputStream;

import java.util.HashMap;

public class ReaderResult {

    private final ReaderInputStream reader;
    private final byte status;
    private final String errorMessage;
    private final HashMap<String, String> metadata;

    public ReaderResult(ReaderInputStream reader) {
        this.reader = reader;
        this.status = 0;
        this.errorMessage = null;
        this.metadata = null;
    }

    public ReaderResult(ReaderInputStream reader, HashMap<String, String> metadata) {
        this.reader = reader;
        this.status = 0;
        this.errorMessage = null;
        this.metadata = metadata;
    }

    public ReaderResult(byte status, String errorMessage) {
        this.reader = null;
        this.status = status;
        this.errorMessage = errorMessage;
        this.metadata = null;
    }

    /**
     * Returns the result Reader or null if there is an error
     * @return Reader reader
     */
    public ReaderInputStream getReader() {
        return reader;
    }

    /**
     * Returns the metadata HashMap or null if there is an error
     * @return HashMap metadata
     */
    public HashMap<String, String> getMetadata() {
        return metadata;
    }

    public boolean isError() {
        return status != 0;
    }

    /**
     * Returns the status of the call
     * @return
     * 0: OK
     * 1: IOException
     */
    public byte getStatus() {
        return status;
    }

    /**
     * Returns the error message in case of error
     * @return  String representing the error message or
     * null if there is no error
     */
    public String getErrorMessage() {
        return errorMessage;
    }

    public String toString() {
        return "status:" + this.status + " error: " + this.errorMessage + " reader: "+ this.reader;
    }
}