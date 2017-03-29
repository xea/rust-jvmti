import java.io.IOException;

public class Varying {

    public int function() {
        try {
            int a = 2;
            a++;

        } finally {
            int a = 4;

            a ++;
        }

        return 0;
    }

    public int catchAll() {
        try {
            return mayThrow();
        } catch (IOException ex) {
            int a = 0;
            a++;
        }
        return 0;
    }

    public int mayThrow() throws IOException {

        if (getRandom() > 3) {
            throw new IOException("Hello");
        }


        return 0;
    }

    public int getRandom() {
        return 4;
    }
}
