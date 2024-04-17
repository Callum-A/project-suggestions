use crate::{
    config::Config,
    repositories::{project::ProjectRepository, user::UserRepository},
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub user_repo: UserRepository,
    pub project_repo: ProjectRepository,
}
