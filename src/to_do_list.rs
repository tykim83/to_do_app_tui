use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ToDos {
    pub list: HashMap<String, bool>,
}

impl ToDos {
    pub fn new() -> Self {
        Self {
            list: [
                ("Kill Spike".to_string(), false),
                ("Denmark".to_string(), false),
                ("Iceland".to_string(), true),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    pub fn add(&mut self, todo: &str) {
        self.list.insert(todo.to_string(), false);
    }

    pub fn toogle(&mut self, todo: &str) {
        if let Some(v) = self.list.get_mut(todo) { *v = !*v }
    }

    pub fn clear(&mut self) {
        self.list.retain(|_, v| !*v)
    }
}

// you could run another thread and keep the gui separate from the data 
