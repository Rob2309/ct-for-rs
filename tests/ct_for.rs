use ct_for::ct_for;

#[test]
fn basic() {
    let mut val = 0;
    ct_for!(x in [1, 3, 10] do
        val += x;
    );
    assert_eq!(val, 14);
}

#[test]
fn nested() {
    let mut val = 0;
    ct_for!(x in [1, 3, 10] do
        let mut tmp = x;
        ct_for!(y in (2, 4, 6) do
            tmp += y;
        );
        val += tmp;
    );
    assert_eq!(val, 13 + 15 + 22);
}

#[test]
fn do_not_substitute_literals() {
    let mut val = String::new();

    ct_for!(x in [1, 2, 3] do
        val += "x";
    );

    assert_eq!(val, "xxx".to_string());
}

#[test]
fn substitute_members() {
    struct Foo {
        x: i32,
    }

    let foo = Foo{x: 15};

    let mut val = 0;
    ct_for!(a in [x] do
        val += foo.a;
    );
    assert_eq!(val, 15);
}
