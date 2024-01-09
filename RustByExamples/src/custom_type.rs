enum EstruturaBizarraDeNomeGigantesco {
    Test,
    Teste2,
    Tes(i32)
}

impl EstruturaBizarraDeNomeGigantesco  {
    fn x(&self, run: u32) -> u32{
        match self {
            Self::Test => run,
            _ => 5
        }
    }
}

type troquei = EstruturaBizarraDeNomeGigantesco;

fn types() {
    let x = troquei::Teste2;
    //constantes
    const C:i32 = 26;
    static X:u32 = 15;
}