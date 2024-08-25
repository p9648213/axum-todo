use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};

use crate::{
    app_state::AppState,
    middleware::require_authentication::require_authentication,
    routes::{
        ping::ping,
        tasks::{
            create_task::create_task,
            delete_task::soft_delete,
            get_all_task::get_all_tasks,
            get_one_task::get_one_task,
            update_task::{mark_completed, mark_uncompleted, update_task},
        },
        users::{create_user::create_user, login::login, logout::logout},
    },
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/api/v1/users/logout", post(logout))
        .route("/api/v1/tasks", post(create_task))
        .route("/api/v1/tasks", get(get_all_tasks))
        .route("/api/v1/tasks/:task_id", get(get_one_task))
        .route("/api/v1/tasks/:task_id/completed", put(mark_completed))
        .route("/api/v1/tasks/:task_id/uncompleted", put(mark_uncompleted))
        .route("/api/v1/tasks/:task_id", patch(update_task))
        .route("/api/v1/tasks/:task_id", delete(soft_delete))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ))
        .route("/ping", get(ping))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(login))
        .with_state(app_state)
}
