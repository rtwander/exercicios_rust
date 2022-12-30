use chrono::Local;

pub fn data_atual() {
    let t = Local::now();
    println!("O exercicio2 exibe a data atual no formato ISO 8601.");
    println!("Agora veremos a data atual:");
    println!("{}",t);
}