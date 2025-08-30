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

    /// Processo executado
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

struct Escalonador;
impl Escalonador {
    fn escalonar(&self, processos: &mut Vec<Processo>) {
        // Ordena maior contador p menor
        processos.sort_by(|a, b| b.contador_ciclos.cmp(&a.contador_ciclos));
    }
}

fn main() {
    let cpu = CPU;
    let escalonador = Escalonador;

    // Add processos aqui
    let mut fila_de_prontos = vec![
        Processo::new(1, "Buscar espaco no disco", 8),
        Processo::new(2, "Registrar bit", 5),
        Processo::new(3, "Buscar em memoria virtual", 2),
    ];

    println!("--- Iniciando simulacao da CPU com Escalonador ---");
    println!("Estado inicial da fila: {:?}", fila_de_prontos);

    let mut ciclos_totais = 0;
    // Roda ate que lista de processo esteja vazia 
    while !fila_de_prontos.is_empty() {
        ciclos_totais += 1;
        println!("\n--- CICLO DE TEMPO {} ---", ciclos_totais);

        // ordena fila 
        escalonador.escalonar(&mut fila_de_prontos);

        // Executa o primeiro
        if let Some(processo_a_executar) = fila_de_prontos.get_mut(0) {
            println!("CPU selecionou para executar: Processo {} ({})", processo_a_executar.pid, processo_a_executar.nome);
            cpu.executar_ciclo(processo_a_executar);

            // contador zerado remove da fila
            if processo_a_executar.foi_concluido() {
                println!(
                    "*** PROCESSO {} ({}) CONCLUIDO! Removendo da fila. ***",
                    processo_a_executar.pid, processo_a_executar.nome
                );
                fila_de_prontos.remove(0);
            }
        }

        thread::sleep(Duration::from_millis(800));
    }

    println!("\n--- Simulacao finalizada ---");
    println!("Todos os processos foram executados em {} ciclos de tempo.", ciclos_totais);
}
