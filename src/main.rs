#![windows_subsystem = "windows"]

use notify::{RecursiveMode, Watcher, EventKind};
use std::sync::mpsc::channel;
use std::path::Path;
use odbc::*;
use odbc_safe::AutocommitMode;
use chrono::{NaiveDate, Local};
use std::sync::atomic::{AtomicUsize, Ordering};
use regex::Regex;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let conexao = "DRIVER={SQL Server};SERVER=127.0.0.1;DATABASE=TESTE;UID=usuariosql;PWD=senhadificil";
    let a = match create_environment_v3().map_err(|e| e.unwrap()) {
        Ok(env) => env,
        Err(e) => {
            log_error(&format!("Falha ao criar environment: {:?}", e));
            panic!("Falha ao criar environment.");
        }
    };
    let cnx = match a.connect_with_connection_string(conexao) {
        Ok(conn) => conn,
        Err(e) => {
            log_error(&format!("Falha ao conectar: {:?}", e));
            panic!("Falha ao conectar.");
        }
    };

    let diretorio = r"\\127.0.0.1\pasta";
    let (tx, rx) = channel();

    let mut watcher = match notify::recommended_watcher(move |res| {
        match res {
            Ok(event) => tx.send(event).unwrap(),
            Err(e) => log_error(&format!("Erro no watcher: {:?}", e)),
        }
    }) {
        Ok(w) => w,
        Err(e) => {
            log_error(&format!("Erro ao criar watcher: {:?}", e));
            panic!("Erro ao criar watcher.");
        }
    };

    if let Err(e) = watcher.watch(Path::new(diretorio), RecursiveMode::Recursive) {
        log_error(&format!("Erro ao iniciar watcher: {:?}", e));
        panic!("Erro ao iniciar watcher.");
    }

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
            Err(e) => log_error(&format!("Erro ao receber evento: {:?}", e)),
        }
    }
}

fn log_error(message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_line = format!("[{}] {}\n", timestamp, message);
    
    let mut file = match OpenOptions::new()
        .create(true) // Cria o arquivo se não existir
        .append(true) // Adiciona ao final do arquivo
        .open("monitor_erros.log") {
            Ok(f) => f,
            Err(_e) => {
                // Se falhar ao abrir o arquivo, não tem muito o que fazer sem console
                return;
            }
        };
    
    if let Err(_e) = file.write_all(log_line.as_bytes()) {
        // Se falhar ao escrever, ignora silenciosamente (nao tem console mesmo..kkkk)
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
    
    match Statement::with_parent(cnx) {
        Ok(stmt) => {
            let _timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            if let Err(_e) = stmt.exec_direct(&query) {
                log_error(&format!("Erro ao inserir (ID {}): {:?}", ii, _e));
            }
        }
        Err(_e) => log_error(&format!("Falha ao preparar query (ID {}): {:?}", ii, _e)),
    }
}