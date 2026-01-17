use aide::axum::{
    ApiRouter,
    routing::{get_with, post_with},
};
use axum::{Json, response::NoContent};

use application::shared::EntityCreationDTO;

use crate::{registry::AppRegistry, router::book::handlers::*};

pub fn book_router() -> ApiRouter<AppRegistry> {
    ApiRouter::new().nest(
        "/books",
        ApiRouter::new()
            .api_route(
                "/",
                get_with(get_book_list, |op| op.tag("Books")).post_with(create_book, |op| {
                    op.tag("Books").response::<201, Json<EntityCreationDTO>>()
                }),
            )
            .api_route(
                "/{book_id}",
                get_with(get_book_details, |op| op.tag("Books"))
                    .put_with(update_book, |op| {
                        op.tag("Books").response::<204, NoContent>()
                    })
                    .delete_with(delete_book, |op| {
                        op.tag("Books").response::<204, NoContent>()
                    }),
            )
            .api_route(
                "/{book_id}/checkouts",
                get_with(get_checkout_history, |op| op.tag("Books"))
                    .post_with(checkout_book, |op| {
                        op.tag("Books").response::<204, NoContent>()
                    }),
            )
            .api_route(
                "/{book_id}/return",
                post_with(return_book, |op| {
                    op.tag("Books").response::<204, NoContent>()
                }),
            ),
    )
}
