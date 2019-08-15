use crate::session::Session;
use crate::GMsg;
use seed::prelude::*;

#[derive(Default)]
pub struct Model<'a> {
    session: Session,
    author: &'a str,
}

#[derive(Clone)]
pub enum Msg {}

pub fn init<'a>(session: Session, orders: &mut impl Orders<Msg, GMsg>) -> Model<'a> {
    //    orders
    //        .perform_cmd(loading::notify_on_slow_load(
    //            Msg::SlowLoadThresholdPassed,
    //            Msg::Unreachable,
    //        ))
    //        .perform_cmd(request::author::load(
    //            session.viewer().cloned(),
    //            username.clone(),
    //            Msg::AuthorLoadCompleted,
    //        ))
    //        .perform_cmd(fetch_feed(
    //            session.viewer().cloned(),
    //            username.clone(),
    //            SelectedFeed::default(),
    //            PageNumber::default(),
    //        ));

    Model {
        session,
        ..Model::default()
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {}
}
