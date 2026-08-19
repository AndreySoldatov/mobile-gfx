use std::{cell::RefCell, collections::VecDeque};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("User provided textures do not fit on 1024x1024 atlas")]
    TextureAtlasExhausted,
}

thread_local! {
    pub(crate) static ERROR_QUEUE: RefCell<VecDeque<RuntimeError>> = RefCell::new(VecDeque::new());
}

#[allow(dead_code)]
pub(crate) fn report_error(e: RuntimeError) {
    ERROR_QUEUE.with_borrow_mut(|q| q.push_back(e))
}

#[allow(dead_code)]
pub(crate) fn pop_error() -> Option<RuntimeError> {
    ERROR_QUEUE.with_borrow_mut(|q| q.pop_front())
}
