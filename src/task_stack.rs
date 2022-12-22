use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Write},
    path::Path,
    slice::Iter,
};

pub struct TaskStack {
    name: String,
    list: Vec<Task>,
}

pub type Tasks<'a> = Iter<'a, Task>;

pub struct Task {
    pub content: String,
    pub completed: bool,
}

impl TaskStack {
    pub fn new() -> Self {
        Self {
            name: "miscellaneous".to_string(),
            list: vec![],
        }
    }

    pub fn from_file(file_path: &Path) -> Result<Self, Error> {
        let mut list: Vec<Task> = vec![];
        let group_name: &str = match file_path.file_name() {
            Some(os_str) => match os_str.to_str() {
                Some(str) => str,
                None => "[group-name]",
            },
            None => "[group-name]",
        };

        let tasks_file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("no task group named \"{group_name}\""),
                ))
            }
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

        return Ok(TaskStack {
            name: group_name.to_string(),
            list,
        });
    }

    pub fn name(&self) -> String {
        self.name.to_string()
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
                    std::io::ErrorKind::InvalidInput,
                    format!("no task with number {task_index}"),
                ))
            }
        };
        return Ok(());
    }

    pub fn remove(&mut self, task_index: usize) -> Result<(), Error> {
        if task_index >= self.list.len() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                format!("no task with number {task_index}"),
            ));
        }
        self.list.remove(task_index);
        return Ok(());
    }

    pub fn write_to_file(&self, file_path: &Path) -> Result<(), Error> {
        let mut tasks_file = match File::create(file_path) {
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

    pub fn tasks(&self) -> Tasks {
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
        self.list.retain(|t| !t.completed);
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
