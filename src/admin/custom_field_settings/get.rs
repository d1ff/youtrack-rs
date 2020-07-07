imports!();

use crate::admin::get::Admin;

new_type!(CustomFieldSettings);

from!(@Admin
        -> CustomFieldSettings = "customFieldSettings");

use super::custom_field::get::CustomFields;
use super::types::get::Types;

impl_macro!(
    @CustomFieldSettings
    |=> types -> Types
    |
    @CustomFieldSettings
    |=> custom_fields -> CustomFields
    |);
