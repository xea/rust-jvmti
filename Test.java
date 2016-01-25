public class Test {

    public static void main(final String[] args) throws Exception {
        final Test test = new Test();

        test.printSomething();
        test.throwChecked();
        test.throwUnchecked();
    }


    public void printSomething() {
        System.out.println("Lofasz");
    }

    public void throwChecked() {
        try {
            throw new ClassNotFoundException();
        } catch (final ClassNotFoundException e) {
            System.out.println("Caught");
        }
    }

    public void throwUnchecked() throws Exception {
        throw new NullPointerException("Application error");
    }
}
