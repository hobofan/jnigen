import some.pkg.HelloWorld;
import other.OtherInterface;

class Main {
  public static OtherInterface createInstance() {
    HelloWorld instance = new HelloWorld();
    return instance;
  }

  public static void main(String[] args) {
    HelloWorld instance = (HelloWorld) createInstance();
    String output = instance.hello("josh");
    System.out.println(output);
  }
}
