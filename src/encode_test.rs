//examples/encode_test.rs
use crate::encoding::encode;

fn encode_test() {
    encode("encode_test", vec![255, 0, 0, 255, 0, 0, 0, 255], 2, 1);
}

fn main() {
    encode_test();
}
