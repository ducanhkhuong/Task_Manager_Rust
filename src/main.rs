use std::io::{self, Read, Write};
use std::fs::{File, OpenOptions};
use serde::{Serialize, Deserialize};
use chrono::Local;

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    motacongviec: String,
    trangthai: bool,
    nguoitao: String,
}

impl Task {
    fn new(id: u32, motacongviec: String, nguoitao: String) -> Task {
        Task {
            id,
            motacongviec,
            trangthai: false,
            nguoitao,
        }
    }
}

fn save_tasks_to_json(tasks: &Vec<Task>) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(tasks)?;
    let mut file = File::create("database/tasks.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn load_tasks_from_json() -> std::io::Result<Vec<Task>> {
    if let Ok(mut file) = File::open("database/tasks.json") {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let tasks: Vec<Task> = serde_json::from_str(&contents)?;
        Ok(tasks)
    } else {
        Ok(Vec::new())
    }
}

fn log_action(choice: &str, task: &Task) -> std::io::Result<()> {
    let log_entry = format!(
        "[{}] Choice: {}, Task: {{ id: {}, motacongviec: \"{}\", trangthai: {}, nguoitao: \"{}\" }}\n",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        choice,
        task.id,
        task.motacongviec,
        task.trangthai,
        task.nguoitao
    );
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log/tasks.log")?;
    file.write_all(log_entry.as_bytes())?;
    Ok(())
}

fn main() {
    let mut tasks: Vec<Task> = match load_tasks_from_json() {
        Ok(loaded_tasks) => loaded_tasks,
        Err(_) => Vec::new(),
    };

    let mut next_id = tasks.iter().map(|task| task.id).max().unwrap_or(0);

    loop {
        println!("                          ");
        println!("---Task-Manager---");
        println!("1. Thêm công việc");
        println!("2. Xóa công việc");
        println!("3. Tick hoàn thành");
        println!("4. Liệt kê công việc");
        println!("5. Thoát");
        println!("Chọn 1 lựa chọn đi nào <3!");
        println!("                          ");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("[Error] : Not read line");
        let choice = choice.trim();

        match choice {
            "1" => {
                let mut motacongviec = String::new();
                println!("Mode 1: Nhập mô tả công việc: ");
                io::stdin().read_line(&mut motacongviec).expect("[Error] : Not read line");
                let motacongviec = motacongviec.trim().to_string();

                let mut nguoitao = String::new();
                println!("Nhập tên người tạo công việc: ");
                io::stdin().read_line(&mut nguoitao).expect("[Error] : Not read line");
                let nguoitao = nguoitao.trim().to_string();

                next_id += 1;
                let new_task = Task::new(next_id, motacongviec, nguoitao);
                log_action(choice, &new_task).expect("[Error] : Failed to log action");
                tasks.push(new_task);
                println!("[Success] : Công việc đã được thêm");
                save_tasks_to_json(&tasks).expect("[Error] : Failed to save tasks");
            }
            "2" => {
                println!("Mode 2: Nhập id công việc cần xóa");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("[Error] : Not read line");
                match id_str.trim().parse::<u32>() {
                    Ok(id) => {
                        let original_len = tasks.len();
                        if let Some(task) = tasks.iter().find(|t| t.id == id) {
                            log_action(choice, task).expect("[Error] : Failed to log action");
                        }
                        tasks.retain(|task| task.id != id);
                        if tasks.len() < original_len {
                            println!("[Success] : Công việc đã được xóa");
                        } else {
                            println!("[Error] : Không tìm thấy công việc với id: {}", id);
                        }
                        save_tasks_to_json(&tasks).expect("[Error] : Failed to save tasks");
                    }
                    Err(_) => eprintln!("[Error] : ID không hợp lệ. Vui lòng nhập số nguyên."),
                }
            }
            "3" => {
                println!("Mode 3: Nhập id công việc đánh dấu hoàn thành");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("[Error] : Not read line");
                match id_str.trim().parse::<u32>() {
                    Ok(id) => {
                        let mut found = false;
                        for task in &mut tasks {
                            if task.id == id {
                                task.trangthai = true;
                                log_action(choice, task).expect("[Error] : Failed to log action");
                                println!("[Success] : Công việc đã được hoàn thành");
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            eprintln!("[Error] : Không tìm thấy công việc với id: {}", id);
                        }
                        save_tasks_to_json(&tasks).expect("[Error] : Failed to save tasks");
                    }
                    Err(_) => eprintln!("[Error] : ID không hợp lệ. Vui lòng nhập số nguyên."),
                }
            }
            "4" => {
                println!("Mode 4: Danh sách công việc");
                for task in &tasks {
                    let status = if task.trangthai { "Hoàn thành" } else { "Chưa hoàn thành" };
                    println!("{}: {} [{}] - Tên : {}", task.id, task.motacongviec, status, task.nguoitao);
                    log_action(choice, task).expect("[Error] : Failed to log action");
                }
            }
            "5" => break,
            _ => eprintln!("[Error] : Thao tác không thành công"),
        }
    }
}
