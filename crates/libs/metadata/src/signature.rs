use super::*;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Default)]
pub struct Signature {
    kind: ElementType,
    pointers: usize,
    by_ref: bool,
    is_const: bool,
    is_array: bool,
}

impl Signature {
    pub fn new(
     kind: ElementType,
     pointers: usize,
     by_ref: bool,
     is_const: bool,
     is_array: bool) -> Self {
        Self { kind, pointers, by_ref, is_const, is_array }
    }

    pub fn kind(&self) -> &ElementType {
        &self.kind
    }

    pub fn is_generic(&self) -> bool {
        matches!(self.kind, ElementType::GenericParam(_))
    }

    pub fn is_pointer(&self) -> bool {
        self.pointers > 0
    }

    pub fn is_array(&self) -> bool {
        self.is_array
    }

    pub fn is_blittable(&self) -> bool {
        self.pointers > 0 || self.kind().is_blittable()
    }

    pub fn is_udt(&self) -> bool {
        self.pointers == 0 && self.kind().is_udt()
    }

    pub fn has_union(&self) -> bool {
        self.pointers == 0 && self.kind().has_union()
    }

    pub fn has_pack(&self) -> bool {
        self.pointers == 0 && self.kind().has_pack()
    }

    pub fn is_callback(&self) -> bool {
        self.pointers == 0 && self.kind().is_callback()
    }

    pub fn is_callback_array(&self) -> bool {
        self.pointers == 0 && self.kind().is_callback_array()
    }

    pub fn size(&self) -> usize {
        if self.pointers > 0 {
            1
        } else {
            self.kind().size()
        }
    }

    pub fn is_primitive(&self) -> bool {
        self.pointers > 0 || self.kind().is_primitive()
    }
}
