#[cfg(feature = "mariadb")]
pub mod mariadb;

#[cfg(feature = "posgres")]
pub mod postgres;

#[cfg(feature = "sqlite")]
pub mod sqlite;

#[tokio::main]
async fn main() {
    #[cfg(feature = "mariadb")]
    assert!(mariadb::main().await.is_ok());

    #[cfg(feature = "posgres")]
    assert!(postgres::main().await.is_ok());

    #[cfg(feature = "sqlite")]
    assert!(sqlite::main().await.is_ok());
}
