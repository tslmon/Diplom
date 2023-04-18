#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Queryable,
    Identifiable,
    Insertable,
    Associations,
)]
// #[belongs_to(Submission, foreign_key = "submission_id")]
// #[belongs_to(Task, foreign_key = "task_id")]
#[table_name = "users"]
pub struct User {
    pub id: UserId,
    pub fname: String,
    pub lname: String,
    pub gender: String,
    pub email: String,
    pub address: String,
    pub type_: String,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Clone, Default, Debug, Queryable, AsChangeset)]
#[table_name = "users"]
pub struct UserForm {
    pub fname: Option<String>,
    pub lname: Option<String>,
    pub gender: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub type_: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Queryable,
    Identifiable,
    Insertable,
    Associations,
)]
#[table_name = "users"]
pub struct UserData {
    pub id: UserId,
    pub fname: String,
    pub lname: String,
    pub gender: String,
    pub email: String,
    pub address: String,
    pub type_: String,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
pub struct UserError {}

impl ModelErrorMessage for UserError {
    fn uniqueviolation_message(_message: Option<&str>) -> Option<String> {
        if _message.is_some() {
            match _message.unwrap() {
                "ukey_submissionss" => Some(String::from("User already exists")),
                _or => Some(String::from("Unknown unique key violation")),
            }
        } else {
            None
        }
    }
    fn foreignkeyviolation_message(_message: Option<&str>) -> Option<String> {
        if _message.is_some() {
            match _message.unwrap() {
                "fkey_clients_tenant_id" => Some(String::from("User id does not exists")),
                _or => Some(String::from("Unknown foreign key violation")),
            }
        } else {
            None
        }
    }
}
