use crate::GMsg;
use seed::prelude::*;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;

pub fn go_to<Ms: 'static>(route: Route, orders: &mut impl Orders<Ms, GMsg>) {
    seed::push_route(route.clone());
    orders.send_g_msg(GMsg::RoutePushed(route));
}

// ------ Route ------

#[derive(Clone)]
pub enum Route {
    Home,
    Root,
    Posts,
    About,
}

impl<'a> Route {
    pub fn path(&self) -> Vec<&str> {
        use Route::*;
        match self {
            Home | Root => vec![],
            Posts => vec!["posts"],
            About => vec!["about"],
        }
    }
}

impl<'a> fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}", self.path().join("/"))
    }
}

impl<'a> From<Route> for seed::Url {
    fn from(route: Route) -> Self {
        route.path().into()
    }
}

impl<'a> TryFrom<seed::Url> for Route {
    type Error = ();

    fn try_from(url: seed::Url) -> Result<Self, Self::Error> {
        let mut path = url.path.into_iter();

        match path.next().as_ref().map(String::as_str) {
            None | Some("") => Some(Route::Home),
            Some("posts") => Some(Route::Posts),
            Some("about") => Some(Route::About),
            _ => None,
        }
        .ok_or(())
    }
}
