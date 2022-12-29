use chrono::Local;

pub fn data_atual() {
    let t = Local::now();
    println!("Agora veremos a data atual:");
    println!("{}",t);
}