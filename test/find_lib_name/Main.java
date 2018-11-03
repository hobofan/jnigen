import some.pkg.HelloWorld;

class Main {
    // The rest is just regular ol' Java!
    public static void main(String[] args) {
        String output = some.pkg.HelloWorld.hello("josh");
        System.out.println(output);
    }
}
