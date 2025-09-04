use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;


#[derive(Debug, Clone, Copy, PartialEq)] // PartialEq pra comparacoes com ==
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
//XXXXXXXXXX
    pub fn criar_processo_aleatorio(&mut self) -> Processo {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("O tempo voltou para tras!")
            .as_nanos();
        
        thread::sleep(Duration::from_nanos(1));

        let contador_aleatorio = (nanos % 41 + 10) as u32;
        let status_possiveis = [StatusProcesso::Pronto, StatusProcesso::Executando, StatusProcesso::Bloqueado];
        let indice_status = (nanos % 3) as usize;
        let status_aleatorio = status_possiveis[indice_status];
        let nome_processo = format!("Processo_{}", self.proximo_id);
//XXXXXXXXXXX
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

pub struct Escalonador {
    fila_de_processos: Vec<Processo>,
}

impl Escalonador {
    // construtor em vetor
    pub fn new(processos: Vec<Processo>) -> Self {
        Escalonador {
            fila_de_processos: processos,
        }
    }

    pub fn iniciar_execucao(&mut self, cpu: &mut CPU) {
        cpu.executar_processos(&mut self.fila_de_processos);
    }
}

pub struct CPU {}
impl CPU {
    pub fn new() -> Self {
        CPU {}
    }

    // CPU com FCFS
    pub fn executar_processos(&mut self, fila: &mut Vec<Processo>) {
        println!("[CPU]: Iniciando ciclo de execucao FCFS...");
        
        // Itera sobre cada processo da fila, na ordem em que chegaram
        for processo in fila {
            println!("\n----------------------------------------------------");
            println!("[CPU]: Carregando '{}' (ID: {}) com contador: {}", processo.nome, processo.id, processo.contador);
            processo.status = StatusProcesso::Executando; // Muda o status para executando

            while processo.contador > 0 {
                // verificar o status em cada ciclo
                if processo.status == StatusProcesso::Bloqueado {
                    println!("[CPU]: Processo '{}' BLOQUEADO. Pulando 1 ciclo.", processo.nome);
                    thread::sleep(Duration::from_millis(300)); 
                    processo.status = StatusProcesso::Pronto; 
                    println!("[CPU]: Processo '{}' agora esta PRONTO.", processo.nome);
                } else {
                    println!("[CPU]: Executando '{}'... Contador: {}", processo.nome, processo.contador);
                    processo.contador -= 1;
                }
                thread::sleep(Duration::from_millis(200));
            }

            println!("[CPU]: Processo '{}' (ID: {}) concluido!", processo.nome, processo.id);
        }
        println!("[CPU]: Todos os processos foram concluidos.");
    }
}

fn main() {
    println!("--- Simulador de SO ---");
    
    let mut gerador = GeradorDeProcesso::new();
    let mut processos = Vec::new();
    for _ in 0..4 { // quanidade de processos
        processos.push(gerador.criar_processo_aleatorio());
    }

    println!("\n--- Fila de Processos Gerada ---");
    for p in &processos {
        println!("{:?}", p);
    }

    let mut escalonador = Escalonador::new(processos);
    let mut cpu = CPU::new();
    
    escalonador.iniciar_execucao(&mut cpu);
}
