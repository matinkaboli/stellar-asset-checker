use crate::{
    endpoints::horizon::Record,
    utils::{Direction, Endpoint},
    Server,
};

pub trait CallBuilder<'a, T> {
    fn new(s: &'a Server) -> Self;
    fn cursor(&mut self, cursor: &'a str) -> &mut Self;
    fn order(&mut self, dir: Direction) -> &mut Self;
    fn limit(&mut self, limit_number: u8) -> &mut Self;
    fn call(&self) -> Result<Record<T>, anyhow::Error>;
    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self;
}
