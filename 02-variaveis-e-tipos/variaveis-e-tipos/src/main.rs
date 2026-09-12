
/**
*Variaveis
*
*/

fn main() {
    let mut x: i32=10;// signed - positivos ou negativos mut para mutabilidade, trocar depois... 
    let y: u32=8; //unsigned-  nao aceita numeros negativos
    let z: f64 =3.14333;

    let name:String=String::from("Kaua");
    let _name2:&str="kaua";

    println!("x é: {x},y é {y} e z é: {z}");

    //mutabilidade
    x=11;
    println!("x é: {x}");

    println!("Nome é :{name}");


    { //escopo que muda o nome para joao
        let name="Joao";
        let base_calculo=2;
        println!("Nome é :{name}");
        x=1000 * base_calculo;
    }
    // ira quebrar pois foi definida apenas no escopo interno acima
    //println!("Base calculo é {}",base_calculo);
    //escopo da funcao main

    println!("Nome é :{name}");
    println!("x é: {x}");




    //shadowing
    let _idade: i32=20;
    //idade=16;  nao conseguirei mudar o valor em razao da imutabilidade
    let idade: i32=16;  
    println!("idade é: {idade}");
    const TENTATIVAS:i8=10;

    println!("TENTATIVAS é: {TENTATIVAS}");
    
    //char
    let primeria_letra:char='a';
    println!("primeira letra é {primeria_letra}")

    
}
