use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

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

        Processo {
            id: self.proximo_id,
            nome: nome_processo,
            contador: contador_aleatorio,
            status: status_aleatorio,
        }
    }
}


pub struct CPU;

impl CPU {
    pub fn executar_round_robin(&self, processos: &mut Vec<Processo>, quantum: u32) {
        println!("\n[CPU] Iniciando execucao com Round Robin (Quantum = {})...", quantum);

        let mut fila_de_prontos: VecDeque<Processo> = processos.drain(..).collect();

        while let Some(mut processo) = fila_de_prontos.pop_front() {
            println!("\n--- [CPU] Assumindo processo ID: {}, Contador restante: {} ---", processo.id, processo.contador);
            
            let mut ciclos_executados = 0;

            while ciclos_executados < quantum && processo.contador > 0 {
                match processo.status {
                    StatusProcesso::Bloqueado => {
                        println!("[CPU] ID: {}. STATUS: Bloqueado. Processo devolvido para a fila.", processo.id);
                        processo.status = StatusProcesso::Pronto;
                        break; 
                    },
                    _ => { 
                        processo.status = StatusProcesso::Executando;
                        println!("[CPU] Executando: {}. Contador: {} ... {}", processo.id, processo.contador, processo.contador - 1);
                        processo.contador -= 1;
                    }
                }
                
                ciclos_executados += 1;
                thread::sleep(Duration::from_millis(200));
            }

            if processo.contador > 0 {
                println!("--- [CPU] Fim do quantum para ID: {}. Devolvendo ao final da fila. ---", processo.id);
                processo.status = StatusProcesso::Pronto; 
                fila_de_prontos.push_back(processo); 
            } else {
                println!("--- [CPU] Processo ID: {} finalizado! ---", processo.id);
            }
        }

        println!("\n[CPU] Todos os processos foram executados.");
    }
}


fn main() {
    const QUANTUM: u32 = 4;

    let mut gerador = GeradorDeProcesso::new();
    let mut lista_de_processos = Vec::new();
    for _ in 0..4 {
        lista_de_processos.push(gerador.criar_processo_aleatorio());
    }

    println!("\n--- Processos ---");
    for p in &lista_de_processos {
        println!("{:?}", p);
    }

    let cpu = CPU;
    cpu.executar_round_robin(&mut lista_de_processos, QUANTUM);

}
