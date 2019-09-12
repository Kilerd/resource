use crate::GMsg;
use seed::prelude::*;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;
use strum_macros::{Display, EnumString};
use std::str::FromStr;

pub fn go_to<Ms: 'static>(route: Route, orders: &mut impl Orders<Ms, GMsg>) {
    seed::push_route(route.clone());
    orders.send_g_msg(GMsg::RoutePushed(route));
}

// ------ Route ------

#[derive(Clone, Display, EnumString)]
pub enum Route {
    #[strum(serialize="")]
    Home,
    #[strum(serialize="")]
    Root,
    #[strum(serialize="posts")]
    Posts,
    #[strum(serialize="about")]
    About,
}

impl<'a> From<Route> for seed::Url {
    fn from(route: Route) -> Self {
        route.to_string().into()
    }
}

impl<'a> TryFrom<seed::Url> for Route {
    type Error = ();

    fn try_from(url: seed::Url) -> Result<Self, Self::Error> {
        let mut path = url.path.into_iter();
        let x1 = path.next();
        let x = x1.as_ref().map(String::as_str).unwrap_or("");
        Route::from_str(x).map_err(|_|())
    }
}
