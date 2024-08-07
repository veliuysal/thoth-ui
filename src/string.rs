macro_rules! strings {
    ($($name:ident => $content:expr,)*) => (
        $(pub const $name: &str = $content;)*
    )
}

strings! {
  RELOAD_BUTTON => "Reload",
  NEXT_PAGE_BUTTON => "Next page",
  PREVIOUS_PAGE_BUTTON => "Previous",
  PAGINATION_COUNT_BOOKS => "Displaying books",
  SEARCH_WORKS => "Search by title, DOI, internal reference, abstract or landing page",
  RELATIONS_INFO => "Relations below are saved automatically upon change.",
}
