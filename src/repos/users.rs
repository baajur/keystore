use super::error::*;
use diesel::pg::PgConnection;
use models::*;
use prelude::*;
use schema::users::dsl::*;

pub trait UsersRepo {
    fn find_user_by_authentication_token(&self, token: AuthenticationToken) -> Result<Option<User>, Error>;
}

pub struct UsersRepoImpl<'a> {
    db_conn: &'a PgConnection,
}

impl<'a> UsersRepoImpl<'a> {
    pub fn new(db_conn: &'a PgConnection) -> Self {
        UsersRepoImpl { db_conn }
    }
}

impl<'a> UsersRepo for UsersRepoImpl<'a> {
    fn find_user_by_authentication_token(&self, token: AuthenticationToken) -> Result<Option<User>, Error> {
        users
            .filter(authentication_token.eq(token))
            .limit(1)
            .get_result(self.db_conn)
            .optional()
            .map_err(ectx!(ErrorKind::Internal))
    }
}