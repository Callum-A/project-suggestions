use crate::{
    config::Config,
    repositories::{project::ProjectRepository, tag::TagRepository, user::UserRepository},
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub user_repo: UserRepository,
    pub project_repo: ProjectRepository,
    pub tag_repo: TagRepository,
}
