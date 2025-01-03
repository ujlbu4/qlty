class BooleanLogic {
    int zee;
    // bug: this field is not counted in metrics because it has the same name as a field in BooleanLogic1
    int bar;

    void f0() {
        var x = zee - bar + 1;
    }
}

class BooleanLogic1 {
    boolean foo;
    boolean bar;
    boolean baz;
    boolean qux;

    void f1() {
        if (foo && bar && baz && qux) {
            return;
        }
    }
}
