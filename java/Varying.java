public class Varying {

    private Varying() {
    }

    public void method1() {}

    public int method2() { return 13; }

    public int method3(int p) { return 25; }

    public int method4(int p, char q) { return 34; }

    public int method5(int p, char q) {
        int a = p;

        return 45;
    }

    public int method6(int p, char q) {
        method1();

        return 48;
    }

    public int method7(int p, char q) {
        method1();

        int a = p;

        return 48;
    }

    public int method8(int p, char q) {
        method1();

        int a = p;

        method1();

        return 48;
    }
}
