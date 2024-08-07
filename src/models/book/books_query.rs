use crate::models::utils::WorkWithRelations;
use serde::Deserialize;
use serde::Serialize;

pub use crate::models::utils::Variables;

pub const WORKS_QUERY_BODY: &str = "
            workId
            workType
            workStatus
            fullTitle
            title
            landingPage
            doi
            coverUrl
            license
            place
            publicationDate
            withdrawnDate
            updatedAt
            contributions {
                contributionId
                workId
                contributorId
                contributionType
                mainContribution
                createdAt
                updatedAt
                lastName
                fullName
                contributionOrdinal
                contributor {
                    contributorId
                    firstName
                    lastName
                    fullName
                    orcid
                    website
                    createdAt
                    updatedAt
                }
            }
            imprint {
                imprintId
                imprintName
                updatedAt
                publisher {
                    publisherId
                    publisherName
                    publisherShortname
                    publisherUrl
                    createdAt
                    updatedAt
                }
            }
        }";

pub const BOOKS_QUERY_HEADER: &str = "
    query BooksQuery($limit: Int, $offset: Int, $filter: String, $publishers: [Uuid!], $order: WorkOrderBy) {
        books(limit: $limit, offset: $offset, filter: $filter, publishers: $publishers, order: $order) {";

pub const BOOKS_QUERY_FOOTER: &str = "
        bookCount(filter: $filter, publishers: $publishers)
    }
";

graphql_query_builder! {
    BooksRequest,
    BooksRequestBody,
    Variables,
    format!("{BOOKS_QUERY_HEADER}{WORKS_QUERY_BODY}{BOOKS_QUERY_FOOTER}"),
    BooksResponseBody,
    BooksResponseData,
    FetchBooks,
    FetchActionBooks
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BooksResponseData {
    pub books: Vec<WorkWithRelations>,
    pub book_count: i32,
}
