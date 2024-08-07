#![allow(clippy::let_unit_value)]

#[macro_export]
macro_rules! pagination_helpers {
    ($component:ident, $pagination_text:ident, $search_text:ident) => {
        use $crate::string::$pagination_text;
        use $crate::string::$search_text;

        impl $component {
            fn search_text(&self) -> String {
                format!("{}", $search_text)
            }

            fn display_count(&self) -> String {
                let offset_display = match self.offset == 0 && self.result_count > 0 {
                    true => 1,
                    false => self.offset,
                };
                let limit_display = match (self.limit + self.offset) > self.result_count {
                    true => self.result_count,
                    false => self.limit + self.offset,
                };
                format!("{} {}–{} of {}", $pagination_text, offset_display, limit_display, self.result_count)
            }

            fn is_previous_disabled(&self) -> bool {
                self.offset < self.page_size
            }

            fn is_next_disabled(&self) -> bool {
                self.limit + self.offset >= self.result_count
            }

            #[allow(dead_code)]
            fn pagination_controls(&self, ctx: &Context<Self>) -> Html {
                html! {
                <>
                <link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet"/>
                    //              <style>{
                    //     ".table{ border-width: 15px; border-style: solid; border-color: blue;}  "
                    // }
                    <nav style=" display: flex;  justify-content: center;" class="pagination is-centered" role="navigation" aria-label="pagination">
                        <a style="color: #8c3f8d;" class="pagination-previous"
                            onclick={ ctx.link().callback(|_| Msg::PreviousPage) }
                            disabled={ self.is_previous_disabled() }
                        >{ $crate::string::PREVIOUS_PAGE_BUTTON }</a>
                        <a style="color: #8c3f8d;" class="pagination-next"
                            onclick={ ctx.link().callback(|_| Msg::NextPage) }
                            disabled={ self.is_next_disabled() }
                        >{ $crate::string::NEXT_PAGE_BUTTON }</a>
                        <div class="pagination-list">
                            <div class="field" style="width: 80%">
                                <p class="control is-expanded has-icons-left">
                                    <input
                                        class="input"
                                        type="search"
                                        value={ self.search_query.clone() }
                                        placeholder={ self.search_text() }
                                        oninput={ ctx.link().callback(|e: InputEvent| Msg::SearchQueryChanged(e.to_value())) }
                                    />
                                    <span class="icon is-left">
                                        <i class="fas fa-search" aria-hidden="true"></i>
                                    </span>
                                </p>
                            </div>
                        </div>
                    </nav>

                    </>
                }
            }
        }
    }
}

#[macro_export]
macro_rules! pagination_component {
    (
        $component:ident,
        $entity:ty,
        $result:ident,
        $result_count:ident,
        $request:ident,
        $fetch_action:ty,
        $fetch_data:ty,
        $request_body:ident,
        $request_variables:ident,
        $search_text:ident,
        $pagination_text:ident,
        $table_headers:expr,
        $order_struct:ty,
        $order_field:ty,
    ) => {
        use gloo_timers::callback::Timeout;
        use std::str::FromStr;
        use yew::Callback;
        use yew::html;
        use yew::prelude::Component;
        use yew::prelude::Context;
        use yew::prelude::Html;
        use yew::prelude::InputEvent;
        use yew::prelude::Properties;
        use yew_router::history::History;
        use yew_router::prelude::Link;
        use yew_router::prelude::RouterScopeExt;
        use yewtil::fetch::Fetch;
        use yewtil::fetch::FetchAction;
        use yewtil::fetch::FetchState;
        use yewtil::NeqAssign;

        use $crate::component::utils::Loader;
        use $crate::component::utils::Reloader;
        use $crate::route::AppRoute;
        use $crate::models::{CreateRoute, EditRoute, MetadataTable};
        use $crate::models::utils::ThothError;
        use $crate::DEFAULT_DEBOUNCING_TIMEOUT;

        pub struct $component {
            limit: i32,
            offset: i32,
            page_size: i32,
            search_callback: Callback<()>,
            search_query: String,
            debounce_timeout: Option<Timeout>,
            order: $order_struct,
            data: Vec<$entity>,
            table_headers: Vec<String>,
            result_count: i32,
            fetch_data: $fetch_data,
            // Store props value locally in order to test whether it has been updated on props change
        }

        pagination_helpers! {$component, $pagination_text, $search_text}

        pub enum Msg {
            SetFetchState($fetch_action),
            GetData,
            PaginateData,
            SearchQueryChanged(String),
            NextPage,
            PreviousPage,
            ChangeRoute(AppRoute),
            SortColumn($order_field),
        }

        #[derive(PartialEq, Eq, Properties)]
        pub struct Props {
        }

        impl Component for $component {
            type Message = Msg;
            type Properties = Props;

            fn create(ctx: &Context<Self>) -> Self {
                let offset: i32 = Default::default();
                let page_size: i32 = 20;
                let limit: i32 = page_size;
                let search_callback = ctx.link().callback(|_| Msg::PaginateData);
                let search_query: String = Default::default();
                let order = Default::default();
                let result_count: i32 = Default::default();
                let data = Default::default();
                let fetch_data = Default::default();
                let table_headers = $table_headers;
                // Store props value locally in order to test whether it has been updated on props change

                ctx.link().send_message(Msg::PaginateData);

                $component {
                    limit,
                    offset,
                    page_size,
                    search_callback,
                    search_query,
                    debounce_timeout: None,
                    order,
                    data,
                    table_headers,
                    result_count,
                    fetch_data,
                }
            }

            fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
                match msg {
                    Msg::SetFetchState(fetch_state) => {
                        self.fetch_data.apply(fetch_state);
                        self.data = match self.fetch_data.as_ref().state() {
                            FetchState::Fetched(body) => body.data.$result.clone(),
                            _ => Default::default(),
                        };
                        self.result_count = match self.fetch_data.as_ref().state() {
                            FetchState::Fetched(body) => body.data.$result_count,
                            _ => Default::default(),
                        };
                        true
                    }
                    Msg::GetData => {
                        ctx.link()
                            .send_future(self.fetch_data.fetch(Msg::SetFetchState));
                        ctx.link()
                            .send_message(Msg::SetFetchState(FetchAction::Fetching));
                        false
                    }
                    Msg::PaginateData => {
                        let filter = self.search_query.clone();
                        let order = self.order.clone();
                        let body = $request_body {
                            variables: $request_variables {
                                limit: Some(self.limit),
                                offset: Some(self.offset),
                                filter: Some(filter),
                                order: Some(order),
                                publishers: None,
                            },
                            ..Default::default()
                        };
                        let request = $request { body };
                        self.fetch_data = Fetch::new(request);
                        ctx.link().send_message(Msg::GetData);
                        false
                    }
                    Msg::SearchQueryChanged(query) => {
                        self.offset = 0;
                        self.search_query = query;

                        // cancel previous timeout
                        self.debounce_timeout = self.debounce_timeout.take().and_then(|timeout| {
                            timeout.cancel();
                            None
                        });
                        // start new timeout
                        let search_callback = self.search_callback.clone();
                        let timeout = Timeout::new(DEFAULT_DEBOUNCING_TIMEOUT, move || {
                            search_callback.emit(());
                        });
                        self.debounce_timeout = Some(timeout);
                        false
                    }
                    Msg::NextPage => {
                        if self.limit < self.result_count && !self.is_next_disabled() {
                            self.offset += self.page_size;
                            ctx.link().send_message(Msg::PaginateData);
                        }
                        false
                    }
                    Msg::PreviousPage => {
                        if self.offset > 0 && !self.is_previous_disabled() {
                            self.offset -= self.page_size;
                            ctx.link().send_message(Msg::PaginateData);
                        }
                        false
                    }
                    Msg::ChangeRoute(r) => {
                        ctx.link().history().unwrap().push(r);
                        false
                    }
                    Msg::SortColumn(header) => {
                        // Clicking on a header, if enabled, sorts the table by that column ascending
                        // Clicking on the current sort column header reverses the sort direction
                        self.order.direction = match self.order.field.neq_assign(header) {
                            true => Asc,
                            false => match self.order.direction {
                                Asc => Desc,
                                Desc => Asc,
                            },
                        };
                        self.offset = 0;
                        ctx.link().send_message(Msg::PaginateData);
                        false
                    }
                }
            }

            fn changed(&mut self, _ctx: &Context<Self>) -> bool {
                false
            }

            fn view(&self, ctx: &Context<Self>) -> Html {
                let route = <$entity>::create_route();
                html! {
                    <>
                        <nav class="table">
                            <div style="background-color: #ffffff;" class="level-left">
                                <p class="level-item">
                                    <span>
                                    { self.display_count() }
                                    </span>
                                </p>
                            </div>
                            <div class="level-right">
                                <p class="level-item">
                                    <a href="tlksoft.com" />
                                </p>
                            </div>
                        </nav>
                        { self.pagination_controls(ctx) }
                        {
                            match self.fetch_data.as_ref().state() {
                                FetchState::NotFetching(_) => {
                                    html! {<Reloader onclick={ ctx.link().callback(|_| Msg::GetData) }/>}
                                },
                                FetchState::Fetching(_) => html! {<Loader/>},
                                FetchState::Fetched(_body) => html! {
                                    <div class="flex flex-col gap-6">
                                        {
                                            for self.data.iter().map(|r| {
                                                let route = r.edit_route().clone();
                                                r.as_table_row(
                                                ctx.link().callback(move |_| { Msg::ChangeRoute(route.clone()) }))
                                            })
                                        }
                                    </div>
                                },
                                FetchState::Failed(_, err) => html! {
                                    { ThothError::from(err).to_string() }
                                },
                            }
                        }
                    </>
                }
            }
        }
    };
}

pub trait ToOption {
    fn to_opt_string(self) -> Option<String>;
    fn to_opt_float(self) -> Option<f64>;
    fn to_opt_int(self) -> Option<i32>;
    fn to_opt_date(self) -> Option<chrono::NaiveDate>;
}

impl ToOption for String {
    fn to_opt_string(self) -> Option<String> {
        match self.trim().is_empty() {
            true => None,
            false => Some(self.trim().to_owned()),
        }
    }

    fn to_opt_float(self) -> Option<f64> {
        let value = self.parse().unwrap_or(0.0);
        match value == 0.0 {
            true => None,
            false => Some(value),
        }
    }

    fn to_opt_int(self) -> Option<i32> {
        let value = self.parse().unwrap_or(0);
        match value == 0 {
            true => None,
            false => Some(value),
        }
    }

    fn to_opt_date(self) -> Option<chrono::NaiveDate> {
        match chrono::NaiveDate::parse_from_str(&self, "%Y-%m-%d") {
            Ok(date) => Some(date),
            Err(_) => None,
        }
    }
}

pub trait ToElementValue {
    fn to_value(self) -> String;
}

impl ToElementValue for yew::InputEvent {
    fn to_value(self) -> String {
        use wasm_bindgen::JsCast;
        use web_sys::{HtmlInputElement, HtmlTextAreaElement};
        let target = self.target().expect("Failed to get InputEvent target");
        if target.has_type::<HtmlInputElement>() {
            target.unchecked_into::<HtmlInputElement>().value()
        } else if target.has_type::<HtmlTextAreaElement>() {
            target.unchecked_into::<HtmlTextAreaElement>().value()
        } else {
            // We currently only expect to encounter Input and TextArea elements from InputEvents
            unimplemented!()
        }
    }
}

impl ToElementValue for yew::Event {
    fn to_value(self) -> String {
        use wasm_bindgen::JsCast;
        use web_sys::HtmlSelectElement;
        let target = self.target().expect("Failed to get Event target");
        if target.has_type::<HtmlSelectElement>() {
            target.unchecked_into::<HtmlSelectElement>().value()
        } else {
            // We currently only expect to encounter Select elements from Events
            unimplemented!()
        }
    }
}

impl ToElementValue for Option<chrono::NaiveDate> {
    fn to_value(self) -> String {
        match self {
            None => "".to_string(),
            Some(date) => date.format("%Y-%m-%d").to_string(),
        }
    }
}

pub mod book;
pub mod books;
pub mod navbar;
pub mod root;
pub mod utils;
