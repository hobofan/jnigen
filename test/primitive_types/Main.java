import some.pkg.HelloWorld;

class Main {
    // The rest is just regular ol' Java!
    public static void main(String[] args) {
        HelloWorld instance = new HelloWorld();

        byte increasedByte = instance.plusOneByte((byte)1);
        System.out.println(increasedByte);

        char increasedChar = instance.plusOneChar('A');
        System.out.println(increasedChar);

        int increasedInt = instance.plusOneInt(1);
        System.out.println(increasedInt);

        short increasedShort = instance.plusOneShort((short)1);
        System.out.println(increasedShort);

        long increasedLong = instance.plusOneLong(1);
        System.out.println(increasedLong);

        boolean flippedBoolean = instance.flipBoolean(false);
        System.out.println(flippedBoolean);

        boolean flippedBoolean2 = instance.flipBoolean(true);
        System.out.println(flippedBoolean2);

        float doubleFloat = instance.twiceFloat(1);
        System.out.println(doubleFloat);

        double doubleDouble = instance.twiceDouble(1);
        System.out.println(doubleDouble);

        instance.doVoid(5);
    }
}
