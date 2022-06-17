use serde::Deserialize;

// Body for the POST '/players' request.
#[derive(Deserialize)]
pub struct CreatePlayerRequest {
    pub name: String,
    pub score: i32,
}

// Body for the PATCH '/players/:id' request.
#[derive(Deserialize)]
pub struct UpdatePlayerRequest {
    pub name: String,
    pub score: i32,
}
