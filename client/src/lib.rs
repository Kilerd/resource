#[macro_use]
extern crate seed;
use crate::route::Route;
use crate::session::Session;
use seed::prelude::*;
use std::convert::TryInto;

mod model;
mod page;
mod route;
mod session;

enum Model {
    //    Profile(page::profile::Model<'a>),
    Home(page::home::Model),
    Posts(page::posts::Model),
    About(page::about::Model),
}

impl Default for Model {
    fn default() -> Self {
        Model::Home(page::home::Model::default())
    }
}

enum Msg {
    RouteChanged(Option<Route>),
    HomeMsg(page::home::Msg),
    PostsMsg(page::posts::Msg),
    About(page::about::Msg),
}

fn init(url: Url, orders: &mut impl Orders<Msg, GMsg>) -> Model {
    orders.send_msg(Msg::RouteChanged(url.try_into().ok()));
    //    Model::Redirect(Session::new(None))
    Model::Home(page::home::Model::default())
}

pub enum GMsg {
    RoutePushed(Route),
    SessionChanged(Session),
}
//fn sink<'a>(g_msg: GMsg, model: &mut Model<'a>, orders: &mut impl Orders<Msg<'static>, GMsg>) {
//    if let GMsg::RoutePushed(ref route) = g_msg {
//        orders.send_msg(Msg::RouteChanged(Some(route.clone())));
//    }
//
//    match model {
//        Model::Home(model) => {
//            page::home::sink(g_msg, model);
//        }
//        Model::Posts(model) => {
//            page::posts::sink(g_msg, model, &mut orders.proxy(Msg::PostsMsg));
//        }
//    }
//}

fn update<'a>(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::RouteChanged(route) => {
            change_model_by_route(route, model, orders);
        }
        Msg::HomeMsg(module_msg) => {
            if let Model::Home(module_model) = model {
                page::home::update(module_msg, module_model, &mut orders.proxy(Msg::HomeMsg));
            }
        }
        Msg::PostsMsg(module_msg) => {
            if let Model::Posts(module_model) = model {
                page::posts::update(module_msg, module_model, &mut orders.proxy(Msg::PostsMsg));
            }
        }
        Msg::About(module_msg) => {
            if let Model::About(module_model) = model {
                page::about::update(module_msg, module_model, &mut orders.proxy(Msg::About))
            }
        }
    }
}

fn change_model_by_route(
    route: Option<Route>,
    model: &mut Model,
    orders: &mut impl Orders<Msg, GMsg>,
) {
    let session = || Session::default();
    if let Some(route) = route {
        match route {
            Route::Root => route::go_to(Route::Home, orders),

            Route::Home => {
                *model = Model::Home(page::home::init(session(), &mut orders.proxy(Msg::HomeMsg)));
            }
            Route::Posts => {
                *model = Model::Posts(page::posts::init(
                    session(),
                    &mut orders.proxy(Msg::PostsMsg),
                ))
            }
            Route::About => {
                *model = Model::About(page::about::init(session(), &mut orders.proxy(Msg::About)))
            }
        }
    };
}

fn view(model: &Model) -> impl View<Msg> {
    match model {
        Model::Home(model) => page::home::view(model).map_message(Msg::HomeMsg),
        Model::Posts(model) => page::posts::view(model).map_message(Msg::PostsMsg),
        Model::About(model) => page::about::view(model).map_message(Msg::About),
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(init, update, view)
        //        .sink(sink)
        .routes(|url| Msg::RouteChanged(url.try_into().ok()))
        .finish()
        .run();
}
