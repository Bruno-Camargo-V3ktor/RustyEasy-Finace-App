use core_app::entitys::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Structs
#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreateDTO {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGetByIdDTO {
    pub id: Uuid,
}

pub struct UserGetByEmailDTO {
    pub email: String,
}

pub struct UserDeleteByIdDTO {
    pub id: Uuid,
}

// Traits
#[async_trait::async_trait]
pub trait CreateUserRepository {
    async fn execute(&self, dto: UserCreateDTO) -> Result<User, ()>;
}

#[async_trait::async_trait]
pub trait GetUserByIdRepository {
    async fn execute(&self, dto: UserGetByIdDTO) -> Result<User, ()>;
}

#[async_trait::async_trait]
pub trait GetUserByEmailRepository {
    async fn execute(&self, dto: UserGetByEmailDTO) -> Result<User, ()>;
}

#[async_trait::async_trait]
pub trait DeleteUserByIdRepository {
    async fn execute(&self, dto: UserDeleteByIdDTO) -> Result<User, ()>;
}
