use std::rc::Rc;

pub enum ProtoString {
    Short(Rc<(u8, [u8; 14])>),

    Long(Rc<str>),
}
