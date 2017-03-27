import java.util.List;
import java.util.ArrayList;


public class LambdaTest<T> {

    public static void main(final String[] args) {
        final LambdaTest<String> test = new LambdaTest<String>();

        test.doStreamProcessing();
    }

    public void doStreamProcessing() {
        final List<String> list = new ArrayList<String>();


        list.add("Apple");
        list.add("Pear");
        list.add("Pineapple");
        list.add("Blueberry");

        list.stream().filter(x -> x.toLowerCase().startsWith("A")).forEach(System.out::println);
    }
}
