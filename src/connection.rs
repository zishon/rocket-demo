use rocket_contrib::databases::diesel;

#[database("rocket_demo")]
pub struct MysqlDatabase(pub diesel::mysql::MysqlConnection);
