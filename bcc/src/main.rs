use bcc::builder::Builder;

fn main() {
    let mut builder = Builder::new();

    let n = builder.new_byte(30);
    let m = builder.new_byte(1);
    let temp1 = builder.n_cells(1);
    let temp2 = builder.n_cells(1);
    let very_empty_space = builder.n_cells(1);
    builder
        // while (n) {
        .goto(n.position)
        .start_while()
        // print(m);
        .copy(m.position, very_empty_space.position)
        .goto(very_empty_space.position)
        .print_as_byte()
        .clear()

        // temp1 = m;
        // temp2 = 3;
        // temp1 %= temp2;
        .copy(m.position, temp1.position)
        .goto(temp2.position)
        .add(3)
        .goto(temp1.position)
        .mod_by()

        // if (temp1) {
        .start_if()
        .just_print("... not multiple of 3")
        .goto(temp1.position)
        // }
        .end_if()
        .just_print("\n")

        // m++;
        .goto(m.position)
        .add(1)
        // n--;
        .goto(n.position)
        .sub(1)
        // }
        .end_while_unchecked();

    println!("{}", builder.finish(false));
}