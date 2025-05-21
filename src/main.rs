mod tests;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Especialidades {
    Cardiologista,
    Geral,
    Endocrionologista
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Atendimento {
    especialidade: Especialidades,
    espera: u16
}

#[derive(Debug, PartialEq, Clone)]
struct Ubs {
    atendimentos: Vec<Atendimento>,  // lista de atendimentos que a UBS fornece e seus respectivos tempos de espera
    endereco: String
}

#[derive(Debug, PartialEq, Clone)]
struct SaoCarlos { ubs_lista: Option<Vec<Ubs>> }

#[derive(Debug, PartialEq, Clone)]
struct SaoPaulo { ubs_lista: Option<Vec<Ubs>> }

#[derive(Debug, PartialEq, Clone, Copy)]
enum RegioesEnum {
    SaoCarlos,  
    SaoPaulo
}

#[derive(Debug, PartialEq, Clone)]
struct Regioes {
    sao_carlos: SaoCarlos, // struct SaoCarlos
    sao_paulo: SaoPaulo // struct SaoPaulo
}

impl Regioes {
    // Retorna o Vec<Ubs> se há um, pode ser que não haja Ubs na região, nesse caso retorna None
    fn get_ubs_list(&self, regiao: RegioesEnum) -> Option<&Vec<Ubs>> {
        match regiao {
            RegioesEnum::SaoCarlos => return self.sao_carlos.ubs_lista.as_ref(),
            RegioesEnum::SaoPaulo => return self.sao_paulo.ubs_lista.as_ref(),
        }
    }

    // Encontra a UBS com o atendimento necessário e menor tempo de espera dentro de uma determinada região, None se há Ubs na área
    fn encontrar_ubs (&self, zona: RegioesEnum, especialidade: Especialidades) -> Option<Ubs> {

        let ubs_list: &Vec<Ubs>;  
        let mut menor_tempo: u16 = std::u16::MAX;
        let mut ubs_escolhida: Option<Ubs> = None;

        ubs_list = self.get_ubs_list(zona)?; // automaticamente retorna None se não houver nenhuma UBS na região
        
        for ubs in ubs_list {
            for espc in &ubs.atendimentos {
                if especialidade == espc.especialidade && espc.espera < menor_tempo {
                    menor_tempo = espc.espera;
                    ubs_escolhida = Some(ubs.clone());
                }
            }
        }
        
        Some(ubs_escolhida)?
    }
}

fn main() {

    let zona = RegioesEnum::SaoCarlos;
    let especialidade = Especialidades::Cardiologista;

    let regioes = Regioes {
        sao_carlos: SaoCarlos { ubs_lista: None },
        sao_paulo: SaoPaulo { ubs_lista: None }
    };

    let ubs_escolhida = regioes.encontrar_ubs(zona, especialidade);  // Ubs ou None
    println!("A Ubs escolhida é {:?}", ubs_escolhida);

}

