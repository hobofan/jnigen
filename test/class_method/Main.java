import some.pkg.HelloWorld;

class Main {
    // The rest is just regular ol' Java!
    public static void main(String[] args) {
        HelloWorld instance = new HelloWorld();
        String output = instance.hello("josh");
        System.out.println(output);
    }
}
