use bcc::builder::Builder;

fn main() {
    let mut builder = Builder::default();

    builder.just_print(
        r#"
888    888          888 888
888    888          888 888
888    888          888 888
8888888888  .d88b.  888 888  .d88b.
888    888 d8P  Y8b 888 888 d88""88b
888    888 88888888 888 888 888  888
888    888 Y8b.     888 888 Y88..88P d8b
888    888  "Y8888  888 888  "Y88P"  88P
                                     8P
                                     "

 .d8888b.   .d8888b.   .d8888b.   .d8888b.  888
d88P  Y88b d88P  Y88b d88P  Y88b d88P  Y88b 888
       888 888    888        888        888 888
     .d88P 888    888      .d88P      .d88P 888
 .od888P"  888    888  .od888P"   .od888P"  888
d88P"      888    888 d88P"      d88P"      Y8P
888"       Y88b  d88P 888"       888"        "
888888888   "Y8888P"  888888888  888888888  888

"#);

    println!("{}", builder.finish());
}