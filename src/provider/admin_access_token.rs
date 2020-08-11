use diesel;
use diesel::prelude::*;
use crate::schema::*;
use crate::models::*;
use crate::connection::*;
//use crate::schema::admin::dsl::*;

pub fn get(id: i32, connection: MysqlDatabase) -> QueryResult<AdminAccessToken> {
    admin_access_token::table.find(id).get_result::<AdminAccessToken>(&(connection.0))
}

pub fn get_by_access_token(access_token: &str, connection: MysqlDatabase) -> QueryResult<AdminAccessToken> {
    admin_access_token::table.filter(admin_access_token::dsl::access_token.eq(access_token)).first(&(connection.0))
}
