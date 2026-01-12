use std::sync::Arc;

use application::{
    book::{command::*, query::*},
    user::command::*,
};
use domain::audit::clock::SystemClock;
use infrastructure::{
    book::{BookQueryServiceImpl, BookRepositoryImpl},
    config::AppConfig,
    database::ConnectionPool,
    user::UserRepositoryImpl,
};

#[derive(Clone)]
pub struct AppRegistry {
    config: Arc<AppConfig>,
    create_book: Arc<CreateBookService>,
    update_book: Arc<UpdateBookService>,
    delete_book: Arc<DeleteBookService>,
    get_book_details: Arc<GetBookDetailsService>,
    get_book_list: Arc<GetBookListService>,
    get_or_create_user: Arc<GetOrCreateActorService>,
}

impl AppRegistry {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = AppConfig::from_env()?;
        let db = ConnectionPool::new(&config.database).await?;
        let clock = Arc::new(SystemClock {});

        let book_repository = Arc::new(BookRepositoryImpl::new(db.clone()));
        let book_queryservice = Arc::new(BookQueryServiceImpl::new(db.clone()));
        let user_repository = Arc::new(UserRepositoryImpl::new(db.clone()));

        let create_book = CreateBookService::new(clock.clone(), book_repository.clone());
        let update_book = UpdateBookService::new(clock.clone(), book_repository.clone());
        let delete_book = DeleteBookService::new(clock.clone(), book_repository.clone());

        let get_book_details = GetBookDetailsService::new(book_queryservice.clone());
        let get_book_list = GetBookListService::new(book_queryservice.clone());

        let get_or_create_user =
            GetOrCreateActorService::new(clock.clone(), user_repository.clone());

        Ok(AppRegistry {
            config: Arc::new(config),
            create_book: Arc::new(create_book),
            update_book: Arc::new(update_book),
            delete_book: Arc::new(delete_book),
            get_book_details: Arc::new(get_book_details),
            get_book_list: Arc::new(get_book_list),
            get_or_create_user: Arc::new(get_or_create_user),
        })
    }

    pub fn config(&self) -> Arc<AppConfig> {
        Arc::clone(&self.config)
    }
    pub fn create_book(&self) -> Arc<CreateBookService> {
        Arc::clone(&self.create_book)
    }
    pub fn update_book(&self) -> Arc<UpdateBookService> {
        Arc::clone(&self.update_book)
    }
    pub fn delete_book(&self) -> Arc<DeleteBookService> {
        Arc::clone(&self.delete_book)
    }
    pub fn get_book_details(&self) -> Arc<GetBookDetailsService> {
        Arc::clone(&self.get_book_details)
    }
    pub fn get_book_list(&self) -> Arc<GetBookListService> {
        Arc::clone(&self.get_book_list)
    }
    pub fn get_or_create_user(&self) -> Arc<GetOrCreateActorService> {
        Arc::clone(&self.get_or_create_user)
    }
}
