pub fn optimize(code: &mut Vec<u8>) {
    let mut inloop = false;
    let mut last_nonpure = code.len();

    let mut i = 0;
    for ch in code.clone() {
        match ch as char {
            '[' => {
                inloop = true;
            }
            ']' => {
                inloop = false;
            }
            '.' | ',' => {
                if !inloop {
                    last_nonpure = i;
                } else {
                    last_nonpure = code.len();
                }
            },
            _ => ()
        }
        i += 1;
    }

    code.truncate(last_nonpure + 1);
}