fn main() {

    let mut lumi_inicial = 1;
    let mut lumi_final = 6;
    let mut circuito = 1;
    let mut i = lumi_inicial;
    
    while i <= lumi_final {
            println!("C{}.L{}:(+)" ,circuito ,i );
            println!("C{}.L{}:(-)" ,circuito ,i );
            i = i + 1;
    }

    lumi_inicial = 7;
    lumi_final = 12;
    circuito = 2;
    i = lumi_inicial;
    
    while i <= lumi_final {
            println!("C{}.L{}:(+)" ,circuito ,i );
            println!("C{}.L{}:(-)" ,circuito ,i );
            i = i + 1;
    }

}
