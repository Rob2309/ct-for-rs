use ct_for::ct_for;

fn main() {
    ct_for!(a in ["5", 6, vec![5, 6, 7]] do
        println!("{:?}", a);
    );

    println!("Printing every combination of [x, y]");
    ct_for!(a in (x, y) do
        ct_for!(b in (x, y) do
            println!("{}, {}", stringify!(a), stringify!(b));
        );
    );
}
