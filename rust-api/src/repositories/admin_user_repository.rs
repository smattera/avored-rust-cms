use diesel::{PgConnection};
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::{prelude::*};
use chrono::NaiveDateTime;
// use diesel::prelude::*;
use serde::{Serialize};
use uuid::Uuid;


use crate::schema::admin_users;
use crate::schema::admin_users::dsl::*;


// pub type Email = String;
// pub type Password = String;

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = admin_users)]
pub struct AdminUser {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = admin_users)]
pub struct NewAdminUser {
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}



pub struct AdminUserRepository {
    pub db: Pool<ConnectionManager<PgConnection>>,
}

impl AdminUserRepository {
    pub fn new(db: Pool<ConnectionManager<PgConnection>> ) -> AdminUserRepository {
        AdminUserRepository { db }
    }

    pub fn all(&self) -> Vec<AdminUser> {
        let conn = &mut self.db.get().unwrap();

        admin_users
            .load(conn)
            .expect("Error loading admin_users")
    }

    pub fn create(&self, admin_user_email: String, admin_user_password: String) -> AdminUser {
        let conn = &mut self.db.get().unwrap();
        let current = chrono::offset::Utc::now().naive_utc();

        let new_admin_user_struct = NewAdminUser { email: admin_user_email, password: admin_user_password, created_at: current, updated_at: current };

        let admin_user_model = diesel::insert_into(admin_users)
            .values(&new_admin_user_struct)
            .get_result::<AdminUser>(conn)
            .expect("Error creating new admin user record");

        admin_user_model
    }
    pub fn find_by_email(&self, admin_user_email: String) -> AdminUser {
        let conn = &mut self.db.get().unwrap();
        let expect_message = format!("Error loading admin_users by email: {}", &admin_user_email);

        let admin_user_model = admin_users
            .filter(email.eq(admin_user_email))
            .first::<AdminUser>(conn)
            .expect(&expect_message);

        admin_user_model
    }
}