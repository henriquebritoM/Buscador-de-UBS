/*
 *  Estrutura de dados feita para localizar UBSs 
 *  com base na região, especialidade médica
 *  e tempo de espera (fila)
 * 
 *  Esse projeto é apenas um protótipo, feito
 *  com intenção educativa como teste de conceito
*/

use std::vec;

//
//-------------------------- Trazendo o módulo de testes para o escopo --------------------------//
//
mod tests;

//-------------------------- Enum Especidalidades --------------------------//
//
//  Esse enum agrupa os vários tipos de atendimento médicos,
//  para futuras filtragens
//  Novas variantes podem ser adicionadas conforme necessário 
#[derive(Debug, PartialEq, Clone, Copy)]
enum Especialidades {
    Cardiologista,
    Geral,
    Endocrionologista
}

//-------------------------- Struct Atendimento --------------------------//
//  
//  Atendimento agrupa uma especialidade médica (enum Especialidades)
//  junto a sua respectiva fila de espera (u16)
#[derive(Debug, PartialEq, Clone, Copy)]
struct Atendimento {
    especialidade: Especialidades,
    espera: u16
}


//-------------------------- Struct Ubs --------------------------//
//
//  Agrupa um endereço (String) e um vetor de vários 
//  Atendimentos
#[derive(Debug, PartialEq, Clone)]
struct Ubs {
    atendimentos: Vec<Atendimento>,  // lista de atendimentos que a UBS fornece e seus respectivos tempos de espera
    endereco: String
}

/************************************************************************
 *                                                                      *
 *              Declaração dos structs das regiões                      *
 *                                                                      *
 ************************************************************************/
 //
 // É importante que cada região tenha sua própria struct,
 // para que possam ser selecionadas individualmentes através
 // da struct Regioes
 // Cada struct de região contém um Option de um vetor de Ubs
 // Todo: remover o option: vetor vazio deve ser usado no lugar
//

#[derive(Debug, PartialEq, Clone)]
struct SaoCarlos { ubs_lista: Vec<Ubs> }

#[derive(Debug, PartialEq, Clone)]
struct SaoPaulo { ubs_lista: Vec<Ubs> }

/*
 *  Fim da declaração das structs de regiões
*/

//-------------------------- Enum RegioesEnum --------------------------//
//
//  Cada variante desse enum deve linkar a uma struct de regiões
//  Esse Enum é para ser usado como entrada do usuário
#[derive(Debug, PartialEq, Clone, Copy)]
enum RegioesEnum {
    SaoCarlos,  
    SaoPaulo
}

//-------------------------- Struct Regioes --------------------------//
//
//  Struct principal
//  Agrupa todos os structs de região para fácil match
//
#[derive(Debug, PartialEq, Clone)]
struct Regioes {
    sao_carlos: SaoCarlos, // struct SaoCarlos
    sao_paulo: SaoPaulo // struct SaoPaulo
}

impl Regioes {
    // Retorna o Vec<Ubs> se há um, pode ser que não haja Ubs na região, nesse caso retorna None
    fn get_ubs_list(&self, regiao: RegioesEnum) -> Option<&Vec<Ubs>> {
        let vec_ubs: &Vec<Ubs>;

        match regiao {
            RegioesEnum::SaoCarlos => vec_ubs = &self.sao_carlos.ubs_lista,
            RegioesEnum::SaoPaulo => vec_ubs = &self.sao_paulo.ubs_lista,
        }

        if !vec_ubs.is_empty() {
            return Some(vec_ubs);
        }
        else {
            return None;
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
        sao_carlos: SaoCarlos { ubs_lista: Vec::new() },
        sao_paulo: SaoPaulo { ubs_lista: Vec::new() }
    };

    let ubs_escolhida = regioes.encontrar_ubs(zona, especialidade);  // Ubs ou None
    println!("A Ubs escolhida é {:?}", ubs_escolhida);

}

