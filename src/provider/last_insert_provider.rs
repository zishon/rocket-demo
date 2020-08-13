use diesel;
use diesel::prelude::*;
use crate::schema::*;
use crate::models::*;
use crate::connection::*;
//use crate::schema::admin::dsl::*;

//pub fn last_insert_id(connection: &MysqlDatabase) -> i32 {
  //  let insert_id = diesel::sql_query("select last_insert_id() as id").load::<AdminAccessToken>(&connection.0).expect("get_id_error").first().unwrap().id;
   // insert_id
//}

pub fn last_insert_id(connection: &MysqlDatabase) -> i32 {
    let result: Vec<InsertId> = diesel::sql_query("select last_insert_id() as id").load(&connection.0).unwrap();
    result[0].id
    //let query = diesel::query_sql("select last_insert_id() as id");
    //let result = query.get_result::<i32>(&connection.0).unwrap();
    //println!("{:}", result);
}
