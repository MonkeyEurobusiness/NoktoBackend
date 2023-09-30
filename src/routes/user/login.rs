use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginData{
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct LoginResponse{
    token: String
}

#[post("/login", format = "json", data = "<data>")]
pub fn login(data: Json<LoginData>) -> Json<LoginResponse> {
    Json(LoginResponse {
        token: data.username.clone()
    })
}