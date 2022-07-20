fn main() {
    let lumi_inicial = 1;
    let lumi_final = 12;
    let circuito = 1;
    let mut i = lumi_inicial;
    while i <= lumi_final {
            println!("C{}.L{}:(+)" ,circuito ,i );
            println!("C{}.L{}:(-)" ,circuito ,i );
            i = i + 1;
    }


}
