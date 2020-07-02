imports!();
use crate::client::GetQueryBuilder;

new_type!(
    Users
    UsersMe
    UsersMeFields);

from!(
    @GetQueryBuilder
        -> Users = "users"
    @Users
        -> UsersMe = "me"
    @UsersMe
        ?> UsersMeFields = "fields");

impl_macro!(
    @Users
    |=> me -> UsersMe
    |
    @UsersMe
    |
    |?> fields -> UsersMeFields = fields);

exec!(UsersMeFields);
