use bcc::builder::Builder;

fn main() {
    let mut builder = Builder::new();

    /*
    a = 35
    b = 50
    c = 4
    a += consume(b)
    a -= consume(c)
    print(a)  // 35 + 50 - 4 = 81
     */

    let a = builder.n_cells(1);
    builder.add(35);
    let b = builder.n_cells(1);
    builder.add(50);
    let c = builder.n_cells(1);
    builder.add(4);

    builder.goto(b.position).add_to(a.position);
    builder.goto(c.position).sub_from(a.position);

    builder.goto(a.position).print_as_byte();

    println!("{}", builder.finish());
}