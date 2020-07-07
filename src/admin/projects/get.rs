imports!();

use crate::admin::get::Admin;

new_type!(
    Projects
    ProjectsTop
    ProjectsTopSkip
    ProjectsTopSkipFields);

from!(
    @Admin
        -> Projects = "projects"
    @Projects
        ?> ProjectsTop = "$top"
    @ProjectsTop
        ?> ProjectsTopSkip = "$skip"
    @ProjectsTopSkip
        ?> ProjectsTopSkipFields = "fields");

impl_macro!(
    @Projects
    |
    |?> top -> ProjectsTop = top
    @ProjectsTop
    |
    |?> skip -> ProjectsTopSkip = skip
    @ProjectsTopSkip
    |
    |?> fields -> ProjectsTopSkipFields = fields);

exec!(ProjectsTopSkipFields);
