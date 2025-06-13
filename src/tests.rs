
#![cfg(test)]

use super::*;

#[test]
fn test_atendimento_struct () {
    let at = Atendimento {
        especialidade:Especialidades::Cardiologista, 
        espera: 12 as u16,
    };

    assert_eq!(at.especialidade, Especialidades::Cardiologista);
    assert_eq!(at.espera, 12 as u16);
}

#[test]
fn test_ubs_struct () {
    let at1 = Atendimento {
        especialidade:Especialidades::Cardiologista, 
        espera: 12 as u16,
    };
    let at2 = Atendimento {
        especialidade:Especialidades::Geral, 
        espera: 4 as u16,
    };
    let at3 = Atendimento {
        especialidade:Especialidades::Endocrionologista, 
        espera: 8 as u16,
    };

    let ubs = Ubs {
        atendimentos: Vec::from([at1, at2, at3]),
        endereco: "Rua UFSCAR, Número 43".to_string()
    };

    assert_eq!(12 as u16, ubs.atendimentos[0].espera);
    assert_eq!(Especialidades::Geral, ubs.atendimentos[1].especialidade);
    assert_eq!(3 as usize, ubs.atendimentos.len());
    assert_eq!("Rua UFSCAR, Número 43".to_string(), ubs.endereco);
}

#[test]
fn test_ubs_sem_atendimentos () {
    let ubs = Ubs {
        atendimentos: Vec::new(),
        endereco: "Rua UFSCAR, Número 43".to_string()
    };

    assert_eq!(0, ubs.atendimentos.len());
    assert_eq!(true, ubs.atendimentos.is_empty());
}

#[test]
#[should_panic(expected ="index out of bounds: the len is 0 but the index is 0")]
fn test_ubs_sem_atendimentos_acessar_vec () {
    let ubs = Ubs {
        atendimentos: Vec::new(),
        endereco: "Rua UFSCAR, Número 43".to_string()
    };

    ubs.atendimentos[0];
}

#[test] 
fn test_sao_carlos_struct () {
    let at1 = Atendimento {
        especialidade:Especialidades::Cardiologista, 
        espera: 12 as u16,
    };
    let at2 = Atendimento {
        especialidade:Especialidades::Geral, 
        espera: 4 as u16,
    };
    let at3 = Atendimento {
        especialidade:Especialidades::Endocrionologista, 
        espera: 8 as u16,
    };

    let ubs = Ubs {
        atendimentos: Vec::from([at1, at2, at3]),
        endereco: "Rua UFSCAR, Número 43".to_string()
    };

    let sao_carlos = SaoCarlos {
        ubs_lista: Vec::from([ubs.clone()]),
    };

    assert_eq!(Vec::from([ubs.clone()]), sao_carlos.ubs_lista);
}

#[test] 
fn test_sao_carlos_struct_empty_list () {

    let sao_carlos = SaoCarlos {
        ubs_lista: Vec::new(),
    };

    assert_eq!(0 as usize, sao_carlos.ubs_lista.len());
}

#[test] 
fn test_regioes_struct () {
    let at1 = Atendimento {
        especialidade:Especialidades::Cardiologista, 
        espera: 12 as u16,
    };
    let at2 = Atendimento {
        especialidade:Especialidades::Geral, 
        espera: 4 as u16,
    };
    let at3 = Atendimento {
        especialidade:Especialidades::Endocrionologista, 
        espera: 8 as u16,
    };

    let ubs = Ubs {
        atendimentos: Vec::from([at1, at2, at3]),
        endereco: "Rua UFSCAR, Número 43".to_string()
    };

    let sao_carlos = SaoCarlos {
        ubs_lista: Vec::from([ubs.clone()]),
    };

    let sao_paulo = SaoPaulo {
        ubs_lista: Vec::new(),
    };

    let regioes = Regioes {
        sao_carlos: sao_carlos,
        sao_paulo: sao_paulo,
    };

    assert_eq!(true, regioes.sao_paulo.ubs_lista.is_empty());
    assert_eq!(Vec::from([ubs.clone()]), regioes.sao_carlos.ubs_lista);
}

#[test]
fn test_regioes_struct_get_ubs_lista () {
    let at1 = Atendimento {
        especialidade:Especialidades::Cardiologista, 
        espera: 12 as u16,
    };
    let at2 = Atendimento {
        especialidade:Especialidades::Geral, 
        espera: 4 as u16,
    };
    let at3 = Atendimento {
        especialidade:Especialidades::Endocrionologista, 
        espera: 8 as u16,
    };

    let ubs = Ubs {
        atendimentos: Vec::from([at1, at2, at3]),
        endereco: "Rua UFSCAR, Número 43".to_string()
    };

    let sao_carlos = SaoCarlos {
        ubs_lista: Vec::from([ubs.clone()]),
    };

    let sao_paulo = SaoPaulo {
        ubs_lista: Vec::new(),
    };

    let regioes = Regioes {
        sao_carlos: sao_carlos,
        sao_paulo: sao_paulo,
    };

    assert_eq!(None, regioes.get_ubs_list(RegioesEnum::SaoPaulo));
    assert_eq!(Especialidades::Cardiologista, regioes.get_ubs_list(RegioesEnum::SaoCarlos).unwrap()[0].atendimentos[0].especialidade);
}

#[test]
fn test_regioes_struct_encontrar_ubs () {
    let at1 = Atendimento {
        especialidade:Especialidades::Cardiologista, 
        espera: 12 as u16,
    };
    let at2 = Atendimento {
        especialidade:Especialidades::Geral, 
        espera: 4 as u16,
    };
    let at3 = Atendimento {
        especialidade:Especialidades::Endocrionologista, 
        espera: 8 as u16,
    };

    let ubs1 = Ubs {
        atendimentos: Vec::from([at1, at2, at3]),
        endereco: "Rua UFSCAR, Número 43".to_string()
    };

    let at4 = Atendimento {
        especialidade:Especialidades::Cardiologista, 
        espera: 5 as u16,
    };
    let at5 = Atendimento {
        especialidade:Especialidades::Geral, 
        espera: 10 as u16,
    };
    let at6 = Atendimento {
        especialidade:Especialidades::Endocrionologista, 
        espera: 8 as u16,
    };

    let ubs2 = Ubs {
        atendimentos: Vec::from([at4, at5, at6]),
        endereco: "Rua USP, Número 65".to_string()
    };

    let regioes = Regioes {
        sao_carlos: SaoCarlos { ubs_lista: Vec::from([ubs1.clone(), ubs2.clone()]) },
        sao_paulo: SaoPaulo { ubs_lista: Vec::new() }
    };

    assert_eq!(Some(ubs2.clone()), regioes.encontrar_ubs(RegioesEnum::SaoCarlos, Especialidades::Cardiologista));

    assert_eq!(Some(ubs1.clone()), regioes.encontrar_ubs(RegioesEnum::SaoCarlos, Especialidades::Geral));

    assert_eq!(Some(ubs1.clone()), regioes.encontrar_ubs(RegioesEnum::SaoCarlos, Especialidades::Endocrionologista));

    assert_eq!(None, regioes.encontrar_ubs(RegioesEnum::SaoPaulo, Especialidades::Cardiologista));
}