use std::{
    collections::HashMap,
    fs::File,
    io::{Error, Write},
};

#[derive(Debug)]
pub struct Tasks {
    pub map: HashMap<usize, Task>,
}

#[derive(Debug)]
pub struct Task {
    pub content: String,
    pub completed: bool,
}

impl Tasks {
    pub fn add(&mut self, content: String) {
        self.map.insert(
            self.map.len() + 1,
            Task {
                content,
                completed: false,
            },
        );
    }

    pub fn complete(&mut self, task_number: usize) -> Result<(), Error> {
        match self.map.get(&task_number) {
            Some(task) => self.map.insert(
                task_number,
                Task {
                    content: task.content.to_string(),
                    completed: true,
                },
            ),
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("no task with number {}", task_number),
                ))
            }
        };
        Ok(())
    }

    pub fn remove(&mut self, task_number: usize) -> Result<(), Error> {
        match self.map.remove(&task_number) {
            Some(_) => Ok(()),
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("no task with number {}", task_number),
                ))
            }
        }
    }

    pub fn write_to_file(&mut self, file_name: &str) -> Result<(), Error> {
        let mut tasks_file = match File::create(file_name) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        for (task_num, Task { content, completed }) in &self.map {
            let task_string = format!("{},{},{}\n", task_num, completed, content);
            match tasks_file.write_all(&task_string.as_bytes()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }
        Ok(())
    }
}
