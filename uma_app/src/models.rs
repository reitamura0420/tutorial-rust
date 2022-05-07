use crate::schema::posts;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Post {
  pub id: i32,
  pub name: String,
  pub evaluation_point: String,
  pub skill_point: String,
  pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
  pub name: &'a str,
  pub evaluation_point: &'a str,
  pub skill_point: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct ChangePost<'a> {
  pub name: Option<&'a str>,
  pub evaluation_point: Option<&'a str>,
  pub skill_point: Option<&'a str>,
}
