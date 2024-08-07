use uuid::Uuid;
//use uuid::Uuid;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum AppRoute {
    #[at("/books")]
    Books,
    #[at("books/:book_id")]
    BookDetail { book_id: Uuid },
    #[not_found]
    #[at("/error")]
    Error,
    #[at("/not-implemented")]
    None,
    #[at("/")]
    Home,
}
