use std::io;

fn main() {
let mut saida0:i32 = 0;
let mut saida1:i32 = 0;
let mut i: i32 = 0;
let mut j: i32 = 0;
let mut saida2: i32 = 0;
let mut conta:Vec<f64> = Vec::new();
while saida0 == 0{
    println!("Escolha qual menu quer acessar");
    println!("1 = Menu de administrador");
    println!("2 = Menu de usuario");
    println!("3 = Sair");
    let mut num = String::new();
    io::stdin().read_line(&mut num)
        .expect("Falha ao ler entrada");
    let num2:i32= num.trim().parse().expect("invalid input");
    if num2 == 1{
        while saida1 == 0{
            println!("1 - Mostrar contas");
            println!("2 - Criar contas");
            println!("3 - Apagar contas");
            println!("4 - Sair");
            let mut num = String::new();
            io::stdin().read_line(&mut num)
                .expect("Falha ao ler entrada");
            let num2:i32= num.trim().parse().expect("invalid input");
            if num2 == 1{
                if i == 0{
                    print!("Nao existem contas criadas");
                    print!("
");
                }
                else{
                    while j <i{
                        println!("Conta{} : Saldo de {}", j, conta[j as usize]); 
                        j = j + 1;
                    }
                    j = 0;
                }
            }
            else if num2 == 2{
                println!("Quantas contas serao criadas?");
                let mut cont = String::new();
                io::stdin().read_line(&mut cont)
                    .expect("Falha ao ler entrada");
                let cont2:i32= cont.trim().parse().expect("invalid input");
                println!("{} serao criadas",cont2);
                while i < cont2{
                conta.push(0.0);
                i = i + 1;
                }
            }
            else if num2 == 3{
                println!("Quantas contas serão apagadas?");
                let mut _del = String::new();
                io::stdin().read_line(&mut _del)
                    .expect("Falha ao ler entrada");
                let _del2:i32= num.trim().parse().expect("invalid input");
                if i == 0{
                    println!("Não existem contas para apagar!");
                }
                else{
                    i = i - num2;
                }
            }
            else if num2 == 4{
                println!("Voce voltara ao menu anterior!");
                saida1 = 1;
            }
            else{
                println!("Numero invalido!")
            }
        }
    }
    else if num2 == 2{
        println!("Qual conta será acessada?");
        let mut acesso1 = String::new();
        io::stdin().read_line(&mut acesso1)
            .expect("Falha ao ler entrada");
        let acesso:i32= acesso1.trim().parse().expect("invalid input");
            if acesso <= i{
                while saida2 == 0{
                    println!("Selecione a operacao");
                    println!("1 = Deposito");
                    println!("2 = Saque");
                    println!("3 = Saldo");
                    println!("4 = Sair");
                    let mut num3 = String::new();
                    io::stdin().read_line(&mut num3)
                        .expect("Falha ao ler entrada");
                    let num4:i32= num3.trim().parse().expect("invalid input");
                    if num4 == 1{      
                        println!("Qual a quantia a ser depositada");
                        let mut dep = String::new();
                        io::stdin().read_line(&mut dep)
                            .expect("Falha ao ler entrada");
                        let dep1:f64= dep.trim().parse().expect("invalid input");
                        conta[acesso as usize] = conta[acesso as usize] + dep1;
                        println!("O saldo e de: {}",conta[acesso as usize]);
                    }else if num4 == 2 {
                        println!("Qual a quantia a ser sacada");
                        let mut saque = String::new();
                        io::stdin().read_line(&mut saque)
                            .expect("Falha ao ler entrada");
                        let saque2:f64= saque.trim().parse().expect("invalid input");
                        if conta[acesso as usize] >= saque2{
                            conta[acesso as usize] = conta[acesso as usize] - saque2;
                            println!("O saldo e de: {}",conta[acesso as usize]);
                        }else{
                            println!("Valor invalido, saque > saldo da conta{}",acesso);
                        }
                    }else if num4 == 3{
                    println!("O saldo da conta{} e de: {}",acesso,conta[acesso as usize]);
                    }else if num4 == 4{
                        println!("Voce saiu com sucesso");
                        saida2 = 1;
                    }else{
                        println! ("Valor invalido");
                    }
                }
            }else{
                println!("Nao existe essa conta!");
            }
    }
    else if num2 == 3{
        println!("Voce saiu com sucesso!");
        saida0 = 1;
    }
    else{
        println! ("Valor invalido");
    }
}
}


