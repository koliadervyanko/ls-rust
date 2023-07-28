use colored::*;
use std::{ fs, env, path::PathBuf, io::{ self, Write, Result } };
use time::OffsetDateTime;

pub struct Commands {}

impl Commands {
    pub fn show(&self) -> Result<PathBuf> {
        env::current_dir()
    }

    pub fn show_content(&self) -> Result<()> {
        let path = self.show()?;

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path: PathBuf = entry.path();
                    let name: String = path.file_name().unwrap().to_string_lossy().into_owned();

                    if path.is_file() {
                        if path.extension().and_then(|s| s.to_str()) == Some("json") {
                            println!("{} ", name.yellow().underline());
                        } else if path.extension().and_then(|s| s.to_str()) == Some("js") {
                            println!("{} ", name.bright_black());
                        } else if path.extension().and_then(|s| s.to_str()) == Some("ts") {
                            println!("{} ", name.blue());
                        } else if path.extension().and_then(|s| s.to_str()) == Some("scss") {
                            println!("{} ", name.bright_red());
                        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                            println!("{} ", name.red());
                        } else {
                            println!("{} ", name);
                        }
                    } else if path.is_dir() {
                        println!("{} ", name.cyan().bold());
                    }
                    io::stdout().flush()?;
                }
            }
            println!();
        }

        Ok(())
    }

    pub fn show_more_content(&self) -> Result<()> {
        let path = self.show()?;

        // Print table header

        if let Ok(entries) = fs::read_dir(path) {
            println!(
                "{:<10} {:<10} {}",
                "Size".bright_white(),
                "Date".bright_white(),
                "Name".bright_white()
            );
            for entry in entries {
                if let Ok(entry) = entry {
                    let path: PathBuf = entry.path();
                    let name = path.file_name().unwrap().to_string_lossy().into_owned();

                    if let Ok(metadata) = fs::metadata(&path) {
                        let size = metadata.len();
                        let datetime = OffsetDateTime::from(metadata.modified().unwrap());
                        let date = datetime.date();

                        if path.is_file() {
                            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                                println!("{:<10} {:<20} {}", size, date, name.yellow().underline());
                            } else if path.extension().and_then(|s| s.to_str()) == Some("js") {
                                println!("{:<10} {:<20} {}", size, date, name.bright_black());
                            } else if path.extension().and_then(|s| s.to_str()) == Some("ts") {
                                println!("{:<10} {:<20} {}", size, date, name.blue());
                            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                                println!("{:<10} {:<20} {}", size, date, name.red());
                            } else {
                                println!("{:<10} {:<20} {}", size, date, name);
                            }
                        }
                        if path.is_dir() {
                            println!("{:<10} {:<20} {}", "-", date, name.cyan().bold());
                        }
                    }

                    io::stdout().flush()?;
                }
            }
            println!();
        }

        Ok(())
    }
}
