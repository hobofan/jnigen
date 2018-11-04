import some.pkg.HelloWorld;

class Main {
    // The rest is just regular ol' Java!
    public static void main(String[] args) {
        HelloWorld instance = new HelloWorld();

        String output = instance.helloInputConversionManual("josh");
        System.out.println(output);

        String output2 = instance.helloInputConversion("josh");
        System.out.println(output2);

        String output3 = instance.helloInputConversionParamName("josh");
        System.out.println(output3);
    }
}
