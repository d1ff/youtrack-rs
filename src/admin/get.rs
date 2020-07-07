imports!();

use crate::client::GetQueryBuilder;

new_type!(Admin);

from!(
    @GetQueryBuilder
        -> Admin = "admin");

use super::custom_field_settings::get::CustomFieldSettings;
use super::projects::get::Projects;

impl_macro!(
    @Admin
    |=> projects -> Projects
    |
    @Admin
    |=> custom_field_settings -> CustomFieldSettings
    |
);
