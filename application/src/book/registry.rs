use std::sync::Arc;

use domain::{audit::Clock, book::interface::BookRepository};

use crate::{
    author::service::AuthorsFactoryService,
    book::{command::*, interface::*, query::*},
};

pub struct BookRegistry {
    create_book: Arc<CreateBookService>,
    update_book: Arc<UpdateBookService>,
    delete_book: Arc<DeleteBookService>,
    checkout_book: Arc<CheckoutBookService>,
    return_book: Arc<ReturnBookService>,
    get_book_details: Arc<GetBookDetailsService>,
    get_book_list: Arc<GetBookListService>,
    get_checkout_history: Arc<GetCheckoutHistoryService>,
}

impl BookRegistry {
    pub fn new(
        repository: Arc<dyn BookRepository>,
        query_service: Arc<dyn BookQueryService>,
        authors_factory_service: Arc<AuthorsFactoryService>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        let create_book = CreateBookService::new(
            clock.clone(),
            repository.clone(),
            authors_factory_service.clone(),
        );
        let update_book = UpdateBookService::new(
            clock.clone(),
            repository.clone(),
            authors_factory_service.clone(),
        );
        let delete_book = DeleteBookService::new(clock.clone(), repository.clone());
        let checkout_book = CheckoutBookService::new(clock.clone(), repository.clone());
        let return_book = ReturnBookService::new(clock.clone(), repository.clone());

        let get_book_details = GetBookDetailsService::new(query_service.clone());
        let get_book_list = GetBookListService::new(query_service.clone());
        let get_checkout_history = GetCheckoutHistoryService::new(query_service.clone());

        BookRegistry {
            create_book: Arc::new(create_book),
            update_book: Arc::new(update_book),
            delete_book: Arc::new(delete_book),
            checkout_book: Arc::new(checkout_book),
            return_book: Arc::new(return_book),
            get_book_details: Arc::new(get_book_details),
            get_book_list: Arc::new(get_book_list),
            get_checkout_history: Arc::new(get_checkout_history),
        }
    }

    pub fn create_book(&self) -> Arc<CreateBookService> {
        self.create_book.clone()
    }

    pub fn update_book(&self) -> Arc<UpdateBookService> {
        self.update_book.clone()
    }

    pub fn delete_book(&self) -> Arc<DeleteBookService> {
        self.delete_book.clone()
    }

    pub fn checkout_book(&self) -> Arc<CheckoutBookService> {
        self.checkout_book.clone()
    }

    pub fn return_book(&self) -> Arc<ReturnBookService> {
        self.return_book.clone()
    }

    pub fn get_book_details(&self) -> Arc<GetBookDetailsService> {
        self.get_book_details.clone()
    }

    pub fn get_book_list(&self) -> Arc<GetBookListService> {
        self.get_book_list.clone()
    }

    pub fn get_checkout_history(&self) -> Arc<GetCheckoutHistoryService> {
        self.get_checkout_history.clone()
    }
}
