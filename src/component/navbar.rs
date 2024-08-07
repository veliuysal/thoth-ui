use yew::html;
use yew::prelude::*;
use yew::virtual_dom::VNode;

pub struct NavbarComponent {}

pub enum Msg {}

#[derive(PartialEq, Properties)]
pub struct Props {}

impl Component for NavbarComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        NavbarComponent {}
    }

    fn view(&self, _ctx: &Context<Self>) -> VNode {
        html! {
            <nav class="navbar is-warning" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <a class="navbar-item" href="/">
                        <img src="https://cdn.thoth.pub/thoth_logo.png" width="50" height="58" style="max-height: none" />
                        <img src="https://cdn.thoth.pub/thoth_name.png" style="margin-left: 0.5em; margin-top: 0.5em" />
                    </a>

                    <a role="button" class="navbar-burger burger" aria-label="menu" aria-expanded="false" data-target="thothNavbar">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    </a>
                </div>

                <div class="navbar-end">
                    <div class="navbar-item">
                        <div class="buttons">
                            <a class="button primary" href="https://github.com/thoth-pub/thoth/blob/master/CHANGELOG.md">
                                {"Website"}
                            </a>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}
