use std::process::id;

use yew::html;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_agent::Dispatched;
use yew_router::prelude::*;

use crate::component::navbar::NavbarComponent;
use crate::route::AppRoute;

use super::book::book::BookDetailComponent;
use super::books::BooksComponent;

pub struct RootComponent {}

pub enum Msg {}

impl Component for RootComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        RootComponent {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        let render = Switch::render(move |r| switch_app(r));

        html! {
            <BrowserRouter>
                <header>
                    <NavbarComponent />
                </header>
                <div class="flex flex-col justify-between min-h-screen text-gray-900 dark:text-gray-100 bg-ternary-50 dark:bg-dark">
                    <Switch<AppRoute> { render } />
                </div>
            </BrowserRouter>
        }
    }
}

fn switch_app(route: &AppRoute) -> Html {
    match route {
        AppRoute::Books => html! {
            <div class="lg:col-span-3 order-2 flex flex-col gap-4">
                <div class="grid grid-cols-12 gap-4 w-full">
                    <div class="col-span-2"></div>
                    <div style="background-color: #ffffff;" class="col-span-8 py-8">
                        <div style="display: flex; justify-content: center; flex-direction: column;" class="section py-12">
                            <BooksComponent />
                        </div>
                    </div>
                    <div class="col-span-2">
                    </div>
                </div>
            </div>
        },

        AppRoute::Home => html! {
            <Redirect<AppRoute> to={ AppRoute::Books }/>
        },
        AppRoute::BookDetail { book_id } => html! {

             <div class="section py-12">
                <BookDetailComponent book_id = {*book_id}/>
             </div>
        },
        AppRoute::Error => html! {
            "Page not found"
        },
        AppRoute::None => html! {
            "Not Implemented"
        },
    }
}
