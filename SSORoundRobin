use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct Processo {
    pid: u32,
    nome: String,
    contador_ciclos: u32,
}

impl Processo {
    fn new(pid: u32, nome: &str, ciclos: u32) -> Self {
        Processo {
            pid,
            nome: String::from(nome),
            contador_ciclos: ciclos,
        }
    }
    fn foi_concluido(&self) -> bool {
        self.contador_ciclos == 0
    }
}

struct CPU;

impl CPU {
    fn executar_ciclo(&self, processo: &mut Processo) {
        if !processo.foi_concluido() {
            processo.contador_ciclos -= 1;
            println!(
                "CPU: Executou ciclo no processo {} ({}). Ciclos restantes: {}",
                processo.pid, processo.nome, processo.contador_ciclos
            );
        }
    }
}

fn main() {
    // Valor do q
    const QUANTUM: u32 = 2;

    let cpu = CPU;

    let mut fila_de_prontos = vec![
        Processo::new(1, "Processo oi", 5),
        Processo::new(2, "Processo deae", 3),
        Processo::new(3, "Processo ggbbbb", 6),
    ];

    println!("Estado inicial da fila: {:?}", fila_de_prontos);

    let mut ciclos_totais = 0;
    while !fila_de_prontos.is_empty() {
        ciclos_totais += 1;
        println!("\n--- TURNO DE ESCALONAMENTO {} ---", ciclos_totais);
        // remove tira da fila
        let mut processo_atual = fila_de_prontos.remove(0);
        println!("Próximo a executar: Processo {} ({})", processo_atual.pid, processo_atual.nome);



        for i in 0..QUANTUM {
            if processo_atual.foi_concluido() {
                println!("Processo terminou antes do fim do quantum.");
                break;
            }
            //executa um ciclo.
            cpu.executar_ciclo(&mut processo_atual);
        }

        if processo_atual.foi_concluido() {
            println!(
                "*** PROCESSO {} ({}) CONCLUÍDO! ***",
                processo_atual.pid, processo_atual.nome
            );
        } else {
            // push bota na fila
            println!(
                "Quantum finalizado. Processo {} ({}) volta para o fim da fila.",
                processo_atual.pid, processo_atual.nome
            );
            fila_de_prontos.push(processo_atual);
        }

        thread::sleep(Duration::from_millis(1200));
    }

    println!("\n--- Simulação finalizada ---");
    println!("Todos os processos foram executados.");
}
