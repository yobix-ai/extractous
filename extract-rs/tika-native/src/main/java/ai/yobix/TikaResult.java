package ai.yobix;

public class TikaResult {

    private final String content;
    private final byte status;
    private final String errorMessage;

    public TikaResult(String content) {
        this.content = content;
        this.status = 0;
        this.errorMessage = null;
    }

    public TikaResult(byte status, String errorMessage) {
        this.content = null;
        this.status = status;
        this.errorMessage = errorMessage;
    }

    /**
     * Returns the result String content or null if there is an error
     * @return String content
     */
    public String getContent() {
        return content;
    }

    public boolean isError() {
        return status != 0;
    }

    /**
     * Returns the status of the call
     * @return
     * 0: OK
     * 1: IOException
     * 2: TikaException
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
        return "status:" + this.status + " error: " + this.errorMessage + "content: "+ this.content;
    }
}
