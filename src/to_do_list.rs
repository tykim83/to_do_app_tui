#[derive(Debug, Clone)]
pub struct ToDos {
    pub list: Vec<ToDo>,
}

impl ToDos {
    pub fn new() -> Self {
        Self {
            list: vec![
                ToDo {
                    name: String::from("Dota"),
                    is_done: true,
                },
                ToDo {
                    name: String::from("Text Game V3"),
                    is_done: false,
                },
            ],
        }
    }

    pub fn add(&mut self, to_do: &str) {
        self.list.push(ToDo {
            name: to_do.to_string(),
            is_done: false,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ToDo {
    pub name: String,
    pub is_done: bool,
}
