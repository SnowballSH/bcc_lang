use bcc::builder::Builder;

fn main() {
    let mut builder = Builder::default();

    builder.just_print(":DD\n");

    let cell = builder.n_cells(1);
    builder.add('A' as u8);

    let cell_copy = builder.n_cells(1);
    builder.copy(cell.position, cell_copy.position);

    builder.goto(cell.position).override_one_byte('G' as u8);

    builder.print_cells(cell);
    builder.print_cells(cell_copy);
    builder.just_print("\n");

    println!("{}", builder.finish());
}