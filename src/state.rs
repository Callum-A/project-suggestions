use crate::{
    config::Config,
    repositories::{project::ProjectRepository, tag::TagRepository, user::UserRepository},
    util::jwt::JWTClient,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub user_repo: UserRepository,
    pub project_repo: ProjectRepository,
    pub tag_repo: TagRepository,
    pub jwt_client: JWTClient,
}
