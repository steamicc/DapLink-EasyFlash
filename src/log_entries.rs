use std::{
    cell::{Ref, RefCell},
    collections::VecDeque,
};

#[derive(Debug, Clone)]
pub enum LogType {
    InfoNoPrefix(String),
    Info(String),
    Warning(String),
    Error(String),
}

#[derive(Debug, Default, Clone)]
pub struct LogEntries {
    entries: RefCell<VecDeque<LogType>>,
}

impl LogEntries {
    pub fn push(&self, log: LogType) {
        self.entries.borrow_mut().push_back(log);
    }

    pub fn pop(&self) -> Option<LogType> {
        self.entries.borrow_mut().pop_front()
    }

    pub fn as_deque(&self) -> Ref<VecDeque<LogType>> {
        self.entries.borrow()
    }
}
