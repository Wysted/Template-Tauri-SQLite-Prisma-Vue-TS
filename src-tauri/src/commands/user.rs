use std::vec;

use crate::{ db::prisma::user, state::AppState };

#[tauri::command(async)]
pub async fn create_user(
    state: tauri::State<'_, AppState>,
    name: String
) -> Result<Vec<user::Data>, String> {
    match
        state.prisma_client
            .user()
            .create(name, vec![])
            .exec().await
    {
        Ok(new_user) => Ok(vec![new_user]),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command(async)]
pub async fn delete_user(
    state: tauri::State<'_, AppState>,
    id: i32
) -> Result<Vec<user::Data>, String> {
    match state.prisma_client.user().delete(user::id::equals(id)).exec().await {
        Ok(del_user) => Ok(vec![del_user]),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command(async)]
pub async fn update_user(
    state: tauri::State<'_, AppState>,
    id: i32,
    name: String
) -> Result<Vec<user::Data>, String> {
    match
        state.prisma_client
            .user()
            .update(user::id::equals(id), vec![user::name::set(name)])
            .exec().await
    {
        Ok(update_user) => Ok(vec![update_user]),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command(async)]
pub async fn get_user(
    state: tauri::State<'_, AppState>,
    id: i32
) -> Result<Vec<user::Data>, String> {
    match state.prisma_client.user().find_unique(user::id::equals(id)).exec().await.unwrap() {
        Some(user) => Ok(vec![user]),
        None => Err("Usuario no encontrado".to_string()),
    }
}
#[tauri::command(async)]
pub async fn get_all_user(state: tauri::State<'_, AppState>) -> Result<Vec<user::Data>, String> {
    match
        state.prisma_client
            .user()
            .find_many(vec![])
            .exec().await
    {
        Ok(users) => Ok(users),
        Err(e) => Err(e.to_string()),
    }
}
