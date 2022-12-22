use crate::{
    traits::Internal,
    BaseData,
};

use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        Storage,
        String,
    },
};

use rmrk_common::{
    error::RmrkError,
    types::*,
};

/// Implement internal helper trait for Base
impl<T> Internal for T
where
    T: Storage<BaseData>,
{
    default fn ensure_only_slot(&self, part_id: PartId) -> Result<Part, PSP34Error> {
        if let Some(part) = self.data::<BaseData>().parts.get(&part_id) {
            if part.part_type != PartType::Slot {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::PartIsNotSlot.as_str(),
                )))
            }
            return Ok(part)
        } else {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::UnknownPartId.as_str(),
            )))
        }
    }
}
