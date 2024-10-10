package ai.yobix;

public class StringResult {

    private final String content;
    private final byte status;
    private final String errorMessage;
    private final String[] metadata;

    public StringResult(String content) {
        this.content = content;
        this.status = 0;
        this.errorMessage = null;
        this.metadata = null;
    }

    public StringResult(String content, String[] metadata) {
        this.content = content;
        this.status = 0;
        this.errorMessage = null;
        this.metadata = metadata;
    }

    public StringResult(byte status, String errorMessage) {
        this.content = null;
        this.metadata = null;
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

    public String[] getMetadata() {
        return this.metadata;
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
        StringBuilder sb = new StringBuilder();
        sb.append("status: ").append(this.status).append("\n");
        if (this.errorMessage != null) {
            sb.append("errorMessage: ").append(this.errorMessage).append("\n");
        } else {
            sb.append("errorMessage: ").append("null").append("\n");
        }
        if (this.content != null) {
            sb.append("content: ").append(this.content).append("\n");
        } else {
            sb.append("content: ").append("null").append("\n");
        }
        if (this.content == null) {
            sb.append("metadata ").append("null").append("\n");
        } else {
            sb.append("metadata ").append("\n");
            for (String metadata: this.metadata) {
                sb.append(metadata).append("\n");
            }
        }
        return sb.toString();
    }
}
