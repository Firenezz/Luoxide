use crate::internal::util::Sealed;

pub trait Visit: Sealed {
    //fn visit<V: Visitor>(&self, visitor: &mut V);
}
