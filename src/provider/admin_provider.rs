use diesel;
use diesel::prelude::*;
use crate::schema::*;
use crate::models::*;
use crate::connection::*;
//use crate::schema::admin::dsl::*;

pub fn get(id: i32, connection: MysqlDatabase) -> QueryResult<Admin> {
    admin::table.find(id).get_result::<Admin>(&(connection.0))
}

pub fn get_by_name(name: &str, connection: &MysqlDatabase) -> QueryResult<Admin> {
    admin::table.filter(admin::dsl::username.eq(name)).first(&connection.0)
}
