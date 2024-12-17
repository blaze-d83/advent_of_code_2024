use std::{collections::HashMap, fs, path::Path};

pub struct DayRegistry {
    solutions: HashMap<String, Box<dyn Fn() -> String>>,
}

impl DayRegistry {
    pub fn new() -> Self {
        let mut registry = DayRegistry {
            solutions: HashMap::new(),
        };
    }

    fn discover_solutions(&mut self) {}

    fn register_day_solutions(&mut self, day_path: &Path) {
        if let Ok(entries) = fs::read_dir(day_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(filename) = path.file_stem() {
                            let day = day_path.file_name().unwrap().to_string_lossy();
                            let part = filename.to_string_lossy();
                            let key = format!("{}_{}", day, part);

                            self.solutions.insert(
                                key,
                                Box::new(|| String::from("Solutions not implemented")),
                            );
                        }
                    }
                }
            }
        }
    }
}
