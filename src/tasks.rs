use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Write},
    slice::Iter,
};

pub struct TaskStack {
    list: Vec<Task>,
}

pub type Tasks<'a> = Iter<'a, Task>;

pub struct Task {
    pub content: String,
    pub completed: bool,
}

impl TaskStack {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn from_file(file_name: &str) -> Result<Self, Error> {
        let mut list: Vec<Task> = vec![];
        let tasks_file = match File::open(file_name) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let reader = BufReader::new(tasks_file);
        for result in reader.lines() {
            match result {
                Ok(line) => {
                    let (_, completed, content) = match parse_task(line) {
                        Ok(tuple) => tuple,
                        Err(e) => return Err(e),
                    };
                    list.push(Task { content, completed });
                }
                Err(e) => return Err(e),
            }
        }

        return Ok(TaskStack { list });
    }

    pub fn add(&mut self, content: String) {
        self.list.push(Task {
            content,
            completed: false,
        });
    }

    pub fn complete(&mut self, task_index: usize) -> Result<(), Error> {
        match self.list.get(task_index) {
            Some(task) => {
                self.list[task_index] = Task {
                    content: task.content.to_string(),
                    completed: true,
                }
            }
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("no task with number {}", task_index),
                ))
            }
        };
        Ok(())
    }

    pub fn remove(&mut self, task_index: usize) -> Result<(), Error> {
        if task_index >= self.list.len() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                format!("no task with number {task_index}, {task_index} is out of bounds"),
            ));
        }
        self.list.remove(task_index);
        Ok(())
    }

    pub fn write_to_file(&mut self, file_name: &str) -> Result<(), Error> {
        let mut tasks_file = match File::create(file_name) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        for (task_index, Task { content, completed }) in self.list.iter().enumerate() {
            let task_string = format!("{},{},{}\n", task_index, completed, content);
            match tasks_file.write_all(&task_string.as_bytes()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }
        Ok(())
    }

    pub fn tasks(&mut self) -> Tasks {
        return self.list.iter();
    }

    pub fn num_tasks(&self) -> usize {
        return self.list.len();
    }

    pub fn num_tasks_completed(&self) -> usize {
        return self.list.iter().fold(0, |acc, cur| {
            return acc + if cur.completed { 1 } else { 0 };
        });
    }

    pub fn remove_completed(&mut self) -> bool {
        let original_length = self.list.len();
        for i in 0..self.list.len() - 1 {
            let index_from_end = original_length - i;
            if self.list[index_from_end].completed {
                self.list.remove(index_from_end);
            }
        }
        return self.list.len() < original_length;
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
