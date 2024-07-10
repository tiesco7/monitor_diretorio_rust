use notify::{RecursiveMode, Watcher, EventKind};
use std::sync::mpsc::channel;
use std::path::Path;
use odbc::*;
use odbc_safe::AutocommitMode;
use chrono::{NaiveDate, Local};
use std::sync::atomic::{AtomicUsize, Ordering};
use regex::Regex;


fn main() {
    let conexao = "DRIVER={SQL Server};SERVER=127.0.0.1;DATABASE=TESTE;UID=usuariosql;PWD=senhadificil";
    let a = create_environment_v3().map_err(|e| e.unwrap()).expect("Falha ao criar environment.");
    let cnx = a.connect_with_connection_string(conexao).expect("Falha ao conectar.");

    let diretorio = r"\\127.0.0.1\pasta";

    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        match res {
            Ok(event) => tx.send(event).unwrap(),
            Err(e) => println!("Erro no watcher: {:?}", e),
        }
    }).unwrap();

    watcher.watch(Path::new(diretorio), RecursiveMode::Recursive).unwrap();

    println!("Iniciado!...");

    let counter = AtomicUsize::new(1);

    loop {
        match rx.recv() {
            Ok(event) => {
                if let EventKind::Create(_) = event.kind {
                    for path in event.paths {
                        if let Some(extension) = path.extension() {
                            if extension == "mp3" || extension == "wav" {
                                let (data_ligacao, telefone, operat) = extrair_dados(&path);
                                let number = counter.fetch_add(1, Ordering::SeqCst);
                                inserir(&cnx, &path.to_string_lossy(), data_ligacao, telefone, operat, number);
                            }
                        }
                    }
                }
            }
            Err(e) => println!("Erro ao receber evento: {:?}", e),
        }
    }
}


fn extrair_dados(path: &Path) -> (Option<String>, Option<String>, Option<String>) {
    let path_str = path.to_str().unwrap_or("");
    let data_ligacao = extrair_data(path_str);
    let telefone = extrair_tel(path_str);
    let operat = extrair_operacao(path_str);
    (data_ligacao, telefone, operat)
}

fn extrair_data(path_str: &str) -> Option<String> {
    let parts: Vec<&str> = path_str.split('-').collect();
    if let Some(last_part) = parts.last() {
        let file_parts: Vec<&str> = last_part.split('.').collect();
        if let Some(date_part) = file_parts.first() {
            if date_part.len() == 14 {
                if let Ok(date) = NaiveDate::parse_from_str(&date_part[..8], "%Y%m%d") {
                    return Some(date.format("%Y-%m-%d").to_string());
                }
            }
        }
    }
    None
}

fn extrair_tel(path_str: &str) -> Option<String> {

    let parts: Vec<&str> = path_str.split('-').collect();
    if let Some(last_part) = parts.first() {
        let file_parts: Vec<&str> = last_part.split('\\').collect();
        if let Some(tel) = file_parts.last() {
            if tel.len() == 10 || tel.len() < 14 {
                return Some(tel.to_string());
            }
        }
    }
    None
}

fn extrair_operacao(path_str: &str) -> Option<String> {
    let parts: Vec<&str> = path_str.split('\\').collect();
    if let Some(part) = parts.get(4) {
        let re = Regex::new(r"[a-zA-Z]").unwrap();
        if re.is_match(part) {
            return Some(part.to_string());
        }
    }
    None

}

fn inserir<AC: AutocommitMode>(cnx: &Connection<'_, AC>, path_do_audio: &str, data_ligacao: Option<String>, telefone: Option<String>, op: Option<String>, ii: usize) {
    let data_lig = data_ligacao.map_or_else(|| "NULL".to_string(), |d| d);
    let tele = telefone.map_or_else(|| "NULL".to_string(), |t| t);
    let operacao = op.map_or_else(|| "NULL".to_string(), |o| o);
    let query = format!("INSERT INTO TESTE.dbo.TABELA (URL, TELEFONE, OP, DATA_LIGACAO) VALUES ('{}', '{}', '{}','{}')", path_do_audio, tele, operacao, data_lig);
    let stmt = Statement::with_parent(cnx).expect("Falha ao preparar query.");
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    match stmt.exec_direct(&query) {
        Ok(_) => println!("{:4} | {} | Arquivo recebido: {:?}", ii, timestamp, path_do_audio),
        Err(e) => println!("Erro ao inserir: {:?}", e),
    }
}
