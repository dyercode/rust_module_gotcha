mod module;

use barebones::module::HiStruct;
use std::any::Any;

fn main() {
    assert_eq!(HiStruct.type_id(), HiStruct.type_id());
    assert_eq!(module::HiStruct.type_id(), module::HiStruct.type_id());
    assert_ne!(module::HiStruct.type_id(), HiStruct.type_id());
}
