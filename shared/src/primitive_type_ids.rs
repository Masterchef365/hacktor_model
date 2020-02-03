use crate::{DataTypeID, HasTypeID};

macro_rules! impl_type_id {
    ($ty: ty, $id: literal) => {
        impl HasTypeID for $ty {
            const TYPE_ID: DataTypeID = $id;
        }
    };
}

impl_type_id!(String, 0xe35f0c8d9d8ecb44);
impl_type_id!(&str, 0xc4fa2d388533b6a1);
