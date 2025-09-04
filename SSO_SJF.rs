use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusProcesso {
    Pronto,
    Executando,
    Bloqueado,
}

#[derive(Debug)]
pub struct Processo {
    pub id: u32,
    pub nome: String,
    pub contador: u32,
    pub status: StatusProcesso,
}

pub struct GeradorDeProcesso {
    proximo_id: u32,
}

impl GeradorDeProcesso {
    pub fn new() -> Self {
        GeradorDeProcesso { proximo_id: 1 }
    }

    pub fn criar_processo_aleatorio(&mut self) -> Processo {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let contador_aleatorio = (nanos % 15 + 5) as u32; 
        let status_possiveis = [StatusProcesso::Pronto, StatusProcesso::Bloqueado, StatusProcesso::Pronto]; 
        let indice_status = (nanos % 3) as usize;
        let status_aleatorio = status_possiveis[indice_status];
        let nome_processo = format!("Processo_{}", self.proximo_id);

        let novo_processo = Processo {
            id: self.proximo_id,
            nome: nome_processo,
            contador: contador_aleatorio,
            status: status_aleatorio,
        };
        self.proximo_id += 1;
        novo_processo
    }
}

pub struct Escalonador;
impl Escalonador {
    pub fn escalonar_sjf(&self, processos: &mut Vec<Processo>) {
        println!("\n[Escalonador] Ordenando processos por menor contador (SJF)...");
        processos.sort_by(|a, b| a.contador.cmp(&b.contador));
    }
}

pub struct CPU;
impl CPU {
    pub fn executar_processos(&self, processos: &mut Vec<Processo>) {
        println!("\n[CPU] Iniciando execucao dos processos...");
        for processo in processos.iter_mut() {
            println!("\n--- [CPU] Carregando processo ID: {}, Nome: '{}', Contador inicial: {} ---", processo.id, processo.nome, processo.contador);
            while processo.contador > 0 {
                match processo.status {
                    StatusProcesso::Bloqueado => {
                        println!("[CPU] Ciclo para ID: {}. STATUS: Bloqueado. Pulando ciclo.", processo.id);
                        processo.status = StatusProcesso::Pronto; 
                    },
                    _ => { 
                        processo.status = StatusProcesso::Executando;
                        println!("[CPU]: Executando '{}'... Contador: {}", processo.id, processo.contador, processo.contador - 1);
                        processo.contador -= 1;
                    }
                }
                thread::sleep(Duration::from_millis(200));
            }
            println!("--- [CPU] Processo ID: {} finalizado! ---", processo.id);
        }
        println!("\n[CPU] Todos os processos foram executados.");
    }
}


fn main() {
    let mut gerador = GeradorDeProcesso::new();
    let mut lista_de_processos = Vec::new();
    for _ in 0..4 {
        lista_de_processos.push(gerador.criar_processo_aleatorio());
    }

    let escalonador = Escalonador;
    escalonador.escalonar_sjf(&mut lista_de_processos);
    
    println!("\n--- Fila de Processos ---");
    for p in &lista_de_processos {
        println!("{:?}", p);
    }

    let cpu = CPU;
    cpu.executar_processos(&mut lista_de_processos);
    println!("\n--- Estado Final dos Processos ---");
    for p in &lista_de_processos {
        println!("{:?}", p);
    }
}
