use core_app::entitys::User;
use serde::{Deserialize, Serialize};

// Structs
#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreateDTO {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

// Traits
#[async_trait::async_trait]
pub trait CreateUserRepository {
    async fn execute(&self, dto: UserCreateDTO) -> Result<User, ()>;
}
