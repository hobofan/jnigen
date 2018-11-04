import some.pkg.HelloWorld;

class Main {
    public static void main(String[] args) {
        HelloWorld instance = new HelloWorld();

        String outputInputConversion = instance.helloInputConversion("josh");
        System.out.println(outputInputConversion);

        String outputInputConversionParamName = instance.helloInputConversionParamName("josh");
        System.out.println(outputInputConversionParamName);

        String outputOutputConversion = instance.helloOutputConversion("josh");
        System.out.println(outputOutputConversion);

        String outputBothConversion = instance.helloBothConversion("josh");
        System.out.println(outputBothConversion);

        // Manual sanity-check

        String outputInputConversionManual = instance.helloInputConversionManual("josh");
        System.out.println(outputInputConversionManual);

        String outputOutputConversionManual = instance.helloOutputConversionManual("josh");
        System.out.println(outputOutputConversionManual);
    }
}
