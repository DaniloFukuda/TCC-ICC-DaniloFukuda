use std::io;
fn main() {
let mut saldo: f32 = 1000.0;
println!("Selecione a operacao");
println!("1 = Deposito");
println!("2 = Saque");
let mut num = String::new();
io::stdin().read_line(&mut num)
        .expect("Falha ao ler entrada");
let num2:i32= num.trim().parse().expect("invalid input");
if num2 == 1{      
    println!("Qual a quantia a ser depositada");
    let mut dep = String::new();
    io::stdin().read_line(&mut dep)
        .expect("Falha ao ler entrada");
    let dep2:f32= dep.trim().parse().expect("invalid input");
    saldo = saldo + dep2;
    println!("O saldo e de: {}",saldo);
    }else if num2 == 2 {
        println!("Qual a quantia a ser sacada");
        let mut saque = String::new();
        io::stdin().read_line(&mut saque)
            .expect("Falha ao ler entrada");
        let saque2:f32= saque.trim().parse().expect("invalid input");
        if saldo >= saque2{
            saldo = saldo - saque2;
            println!("O saldo e de: {}",saldo);
        }else{
            println!("Valor invalido, saque > saldo");
        }
    }else{
        println! ("Valor invalido");
    }
}