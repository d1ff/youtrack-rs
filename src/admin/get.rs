imports!();

use crate::client::GetQueryBuilder;

new_type!(Admin);

from!(
    @GetQueryBuilder
        -> Admin = "admin");

use super::custom_fields_settings::get::CustomFieldsSettings;
use super::projects::get::Projects;

impl_macro!(
    @Admin
    |=> projects -> Projects
    |
    @Admin
    |=> custom_fields_settings -> CustomFieldsSettings
    |
);
