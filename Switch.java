public class Switch {


    public static void main(final String[] args) {

        int a = 0;

        switch (a) {
            case 0:
                a = 1;
                break;
            case 100:
                a = 2;
                break;
            case 2:
                a = 3;
                break;
            case 300:
                a = 4;
                break;
            default:
                a = 5;
                break;
        }

        int b = 1;

        switch (a) {
            case 0:
                b = 3;
                break;
            case 100:
                b = 4;
                break;
            case 200:
                b = 5;
                break;
            case 3:
                b = 6;
                break;
            default:
                b = 7;
                break;
        }


        int c = 2;

        switch (b) {
            case 0:
                c = 3;
                break;
            case 100:
                c = 4;
                break;
            case 200:
                c = 5;
                break;
            case 3:
                c = 6;
                break;
            default:
                c = 7;
                break;
        }



        int d = 3;

        switch (c) {
            case 0:
                d = 3;
                break;
            case 1:
                d = 4;
                break;
            case 200:
                d = 5;
                break;
            case 300:
                d = 6;
                break;
            default:
                d = 7;
                break;
        }

    }

}
