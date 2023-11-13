
use rocket::tokio::time::{sleep, Duration};

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::usuario::Usuario;
use crate::core::domain::value_objects::cpf::Cpf;
use crate::core::domain::value_objects::endereco::Endereco;
use crate ::core::domain::repositories::user_repository::UserRepository;
pub struct InMemoryUserRepository {
  _users: Vec<Usuario>,
}

impl InMemoryUserRepository {
  pub fn new() -> Self {
    let user = Usuario::new(
      1,
      "Albert Dias Moreira".to_string(),
      "contato@albert-dm.dev".to_string(),
      "melhor_projeto".to_string(),
      Cpf { numero: "000.000.000-00".to_string() },
      Endereco { cep: "00000-000".to_string() }
    );
    InMemoryUserRepository {
      _users: vec![user],
    }
  }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
  async fn get_users(&self) -> Result<Vec<Usuario>, DomainError> {
    let users = self._users.clone();
    sleep(Duration::from_secs(1)).await;
    Ok(users)
  }

  async fn get_user_by_id(&self, id: i32) -> Result<Usuario, DomainError> {
    sleep(Duration::from_secs(1)).await;
    for user in &self._users {
      if user.id == id {
        return Ok(user.clone());
      }
    }
    Err(DomainError::NotFound)
  }
}