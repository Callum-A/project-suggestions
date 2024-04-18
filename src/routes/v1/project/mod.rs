use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{model::project::SerializableProject, state::AppState};

const DEFAULT_PER_PAGE: i64 = 10;

#[derive(serde::Deserialize)]
pub struct CreateProjectDTO {
    title: String,
    description: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateProjectDTO {
    title: Option<String>,
    description: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct Paginate {
    page: Option<i64>,
    limit: Option<i64>,
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(payload): Json<CreateProjectDTO>,
) -> Json<SerializableProject> {
    let public_id = uuid::Uuid::new_v4().to_string();
    let project = state
        .project_repo
        .create(&public_id, &payload.title, &payload.description)
        .await;
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
) -> Json<Option<SerializableProject>> {
    // TODO: 404 if project not found
    let project = state.project_repo.find_by_public_id(&public_id).await;
    match project {
        Some(project) => Json(Some(project.into())),
        None => Json(None),
    }
}

pub async fn update_project_by_public_id(
    State(state): State<AppState>,
    Path(public_id): Path<String>,
    Json(payload): Json<UpdateProjectDTO>,
) -> Json<Option<SerializableProject>> {
    // TODO: 404 if project not found, only owner can update
    let project = state.project_repo.find_by_public_id(&public_id).await;
    match project {
        Some(mut project) => {
            if let Some(title) = payload.title {
                project.title = title;
            }
            if let Some(description) = payload.description {
                project.description = description;
            }
            let updated_project = state.project_repo.update(&project).await;
            Json(Some(updated_project.into()))
        }
        None => Json(None),
    }
}

pub async fn delete_project_by_public_id(
    State(state): State<AppState>,
    Path(public_id): Path<String>,
) -> Json<Option<SerializableProject>> {
    // TODO: 404 if project not found, only owner can delete
    let project = state.project_repo.find_by_public_id(&public_id).await;
    match project {
        Some(project) => {
            state.project_repo.delete_by_public_id(&public_id).await;
            Json(Some(project.into()))
        }
        None => Json(None),
    }
}
