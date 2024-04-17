use crate::{config::Config, repositories::user::UserRepository};

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub user_repo: UserRepository,
}
