use axum::{
    extract::{Path, Query, State},
    http::status::StatusCode,
    response::Response,
    Extension, Json,
};

use crate::{
    model::project::SerializableProject,
    state::AppState,
    util::{build_response, jwt::JWTClaims},
};

const DEFAULT_PER_PAGE: i64 = 10;

#[derive(serde::Deserialize)]
pub struct CreateProjectDTO {
    title: String,
    description: String,
    tags: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct UpdateProjectDTO {
    title: Option<String>,
    description: Option<String>,
    tags_to_add: Vec<String>,
    tags_to_remove: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct Paginate {
    page: Option<i64>,
    limit: Option<i64>,
}

pub async fn create_project(
    Extension(user): Extension<JWTClaims>,
    State(state): State<AppState>,
    Json(payload): Json<CreateProjectDTO>,
) -> Json<SerializableProject> {
    let public_id = uuid::Uuid::new_v4().to_string();
    let project = state
        .project_repo
        .create(
            &public_id,
            &payload.title,
            &payload.description,
            user.user_id,
        )
        .await;
    for tag in payload.tags {
        let tag = state.tag_repo.find_by_name_or_create(&tag).await;
        state
            .tag_repo
            .create_tag_to_project(tag.id, project.id)
            .await;
    }
    Json(project.into())
}

pub async fn get_projects(
    State(state): State<AppState>,
    Query(paginate): Query<Paginate>,
) -> Json<Vec<SerializableProject>> {
    let page = paginate.page.unwrap_or(1);
    let limit = paginate.limit.unwrap_or(DEFAULT_PER_PAGE);
    let projects = state.project_repo.paginate(page, limit).await;
    Json(projects.into_iter().map(|project| project.into()).collect())
}

pub async fn get_project_by_public_id(
    State(state): State<AppState>,
    Path(public_id): Path<String>,
) -> Response {
    let project = state.project_repo.find_by_public_id(&public_id).await;
    match project {
        Some(project) => {
            build_response(StatusCode::OK, Json::<SerializableProject>(project.into()))
        }
        None => build_response(StatusCode::NOT_FOUND, "Not Found"),
    }
}

pub async fn update_project_by_public_id(
    Extension(user): Extension<JWTClaims>,
    State(state): State<AppState>,
    Path(public_id): Path<String>,
    Json(payload): Json<UpdateProjectDTO>,
) -> Response {
    let project = state.project_repo.find_by_public_id(&public_id).await;
    match project {
        Some(mut project) => {
            if user.user_id != project.user_id {
                return build_response(StatusCode::UNAUTHORIZED, "Forbidden");
            }

            if let Some(title) = payload.title {
                project.title = title;
            }
            if let Some(description) = payload.description {
                project.description = description;
            }
            for tag in payload.tags_to_add {
                let tag = state.tag_repo.find_by_name_or_create(&tag).await;
                state
                    .tag_repo
                    .create_tag_to_project(tag.id, project.id)
                    .await;
            }
            for tag in payload.tags_to_remove {
                let tag = state.tag_repo.find_by_name_or_create(&tag).await;
                state
                    .tag_repo
                    .delete_tag_to_project(tag.id, project.id)
                    .await;
            }
            let updated_project: SerializableProject =
                state.project_repo.update(&project).await.into();
            build_response(StatusCode::OK, Json(updated_project))
        }
        None => build_response(StatusCode::NOT_FOUND, "Not Found"),
    }
}

pub async fn delete_project_by_public_id(
    Extension(user): Extension<JWTClaims>,
    State(state): State<AppState>,
    Path(public_id): Path<String>,
) -> Response {
    let project = state.project_repo.find_by_public_id(&public_id).await;
    match project {
        Some(project) => {
            if user.user_id != project.user_id {
                return build_response(StatusCode::UNAUTHORIZED, "Forbidden");
            }
            state.project_repo.delete_by_public_id(&public_id).await;
            build_response(StatusCode::OK, Json::<SerializableProject>(project.into()))
        }
        None => build_response(StatusCode::NOT_FOUND, "Not Found"),
    }
}
