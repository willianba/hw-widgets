use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::settings::Settings;
use crate::components::widgets::Widgets;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Widgets,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Router)]
pub fn router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Widgets => html! { <Widgets /> },
        Route::Settings => html! { <Settings /> },
        Route::NotFound => html! { <h1>{"404"}</h1> },
    }
}
