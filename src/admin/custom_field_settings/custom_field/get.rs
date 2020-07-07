imports!();

use crate::admin::custom_field_settings::get::CustomFieldSettings;

new_type!(
    CustomFields
    CustomFieldsTop
    CustomFieldsTopSkip
    CustomFieldsTopSkipFields);

from!(
    @CustomFieldSettings
    -> CustomFields = "customFields"
    @CustomFields
    ?> CustomFieldsTop = "$top"
    @CustomFieldsTop
    ?> CustomFieldsTopSkip = "$skip"
    @CustomFieldsTopSkip
    ?> CustomFieldsTopSkipFields = "fields");

impl_macro!(
    @CustomFields
    |
    |?> top -> CustomFieldsTop = top
    @CustomFieldsTop
    |
    |?> skip -> CustomFieldsTopSkip = skip
    @CustomFieldsTopSkip
    |
    |?> fields -> CustomFieldsTopSkipFields = fields);

exec!(CustomFieldsTopSkipFields);
