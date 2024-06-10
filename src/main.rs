use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use factory::{get_factory, client_code};

#[derive(HelloMacro)]
struct SOMESTRUCT;


fn main() {
    let factory = get_factory();
    client_code(factory);
}

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s
        }

        let transition_syms = num_rows - 2;
        let mut zig_zag_vec: Vec<Vec<String>> = Vec::new();

        let mut start_transition_syms = 0;
        let mut vec_to_push = Vec::new();
        for (index, val) in s.chars().enumerate() {
            if index as i32 % num_rows == 0 {
                vec_to_push.push(val);
            }
            else if start_transition_syms < transition_syms {

            }
            else if start_transition_syms == transition_syms {

            }

        }
        s
    }
}
