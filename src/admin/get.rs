imports!();

use crate::client::GetQueryBuilder;

new_type!(Admin);

from!(
    @GetQueryBuilder
        -> Admin = "admin");

use super::projects::get::Projects;

impl_macro!(
    @Admin
    |=> projects -> Projects
    |
);
