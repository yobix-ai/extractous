package ai.yobix;

import org.apache.commons.io.input.ReaderInputStream;

public class ReaderResult {

    private final ReaderInputStream reader;
    private final byte status;
    private final String errorMessage;

    public ReaderResult(ReaderInputStream reader) {
        this.reader = reader;
        this.status = 0;
        this.errorMessage = null;
    }

    public ReaderResult(byte status, String errorMessage) {
        this.reader = null;
        this.status = status;
        this.errorMessage = errorMessage;
    }

    /**
     * Returns the result Reader or null if there is an error
     * @return Reader reader
     */
    public ReaderInputStream getReader() {
        return reader;
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
