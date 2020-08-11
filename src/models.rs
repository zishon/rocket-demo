use crate::schema::*;

#[derive(Debug)]
#[derive(Queryable)]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub passwd: String,
    pub salt: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[table_name="admin"]
pub struct InsertableAdmin<'a> {
    pub username: &'a str,
    pub passwd: &'a str,
    pub salt: &'a str,
}

#[derive(Debug)]
#[derive(Queryable)]
pub struct AdminAccessToken {
    pub id: i32,
    pub admin_id: i32,
    pub access_token: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[table_name="admin_access_token"]
pub struct InsertableAdminAccessToken<'a> {
    pub admin_id: i32,
    pub access_token: &'a str,
}

#[derive(Queryable)]
pub struct ApiUserSecret {
    pub id: i32,
    pub app_code: String,
    pub app_secret: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[table_name="api_user_secret"]
pub struct InsertableApiUserSecret<'a> {
    pub app_code: &'a str,
    pub app_secret: &'a str,
}
