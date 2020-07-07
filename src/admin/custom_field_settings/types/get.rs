imports!();

use crate::admin::custom_field_settings::get::CustomFieldSettings;

new_type!(
    Types
    TypesTop
    TypesTopSkip
    TypesTopSkipFields);

from!(
    @CustomFieldSettings
    -> Types = "types"
    @Types
    ?> TypesTop = "$top"
    @TypesTop
    ?> TypesTopSkip = "$skip"
    @TypesTopSkip
    ?> TypesTopSkipFields = "fields");

impl_macro!(
    @Types
    |
    |?> top -> TypesTop = top
    @TypesTop
    |
    |?> skip -> TypesTopSkip = skip
    @TypesTopSkip
    |
    |?> fields -> TypesTopSkipFields = fields);

exec!(TypesTopSkipFields);
