package ai.yobix;

import java.io.IOException;
import java.io.InputStream;
import java.nio.ByteBuffer;

public class ByteBufferInputStream extends InputStream {

    private ByteBuffer bb;

    public ByteBufferInputStream(ByteBuffer bb) {
        this.bb = bb;
    }

    @Override
    public int read() throws IOException {
        if (bb == null) {
            throw new IOException("read on a closed InputStream");
        }

        if (bb.remaining() == 0) {
            return -1;
        }

        return (bb.get() & 0xFF);   // need to be in the range 0 to 255
    }

    @Override
    public int read(byte[] b, int off, int len) throws IOException {

        if (bb == null) {
            throw new IOException("read on a closed InputStream");
        }

        if (b == null) {
            throw new NullPointerException();
        } else if (off < 0 || len < 0 || len > b.length - off) {
            throw new IndexOutOfBoundsException();
        } else if (len == 0) {
            return 0;
        }

        int length = Math.min(bb.remaining(), len);
        if (length == 0) {
            return -1;
        }

        bb.get(b, off, length);
        return length;
    }

    @Override
    public long skip(long n) throws IOException {

        if (bb == null) {
            throw new IOException("skip on a closed InputStream");
        }

        if (n <= 0) {
            return 0;
        }

        /*
         * ByteBuffers have at most an int, so lose the upper bits.
         * The contract allows this.
         */
        int nInt = (int) n;
        int skip = Math.min(bb.remaining(), nInt);

        bb.position(bb.position() + skip);

        return nInt;
    }

    @Override
    public int available() throws IOException {

        if (bb == null) {
            throw new IOException("available on a closed InputStream");
        }

        return bb.remaining();
    }

    @Override
    public void close() throws IOException {
        bb = null;
    }

}
