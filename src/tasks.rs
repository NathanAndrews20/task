use std::{
    collections::{HashMap, hash_map::Iter},
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Write},
};

#[derive(Debug)]
pub struct TaskStack {
    pub map: HashMap<usize, Task>,
}

pub type Tasks<'a> = Iter<'a, usize, Task>;
#[derive(Debug)]
pub struct Task {
    pub content: String,
    pub completed: bool,
}

impl TaskStack {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn from_file(file_name: &str) -> Result<Self, Error> {
        let mut map: HashMap<usize, Task> = HashMap::new();
        let tasks_file = match File::open(file_name) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut reader = BufReader::new(tasks_file);
        let mut line = String::new();
        loop {
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    let (num, completed, content) = match parse_task(line.clone()) {
                        Ok(tuple) => tuple,
                        Err(e) => return Err(e),
                    };
                    map.insert(num, Task { content, completed });
                    line.clear()
                }
                Err(e) => return Err(e),
            }
        }

        return Ok(TaskStack { map });
    }

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

    pub fn tasks(&mut self) -> Tasks {
        return self.map.iter();
    }
}

fn parse_task(line: String) -> Result<(usize, bool, String), Error> {
    let task_data_vec: Vec<&str> = line.splitn(3, ",").collect();

    let task_num = match task_data_vec[0].parse::<usize>() {
        Ok(b) => b,
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
    };

    let task_completed = match task_data_vec[1].parse::<bool>() {
        Ok(b) => b,
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
    };

    let task_content = task_data_vec[2].trim().to_string();

    return Ok((task_num, task_completed, task_content));
}
