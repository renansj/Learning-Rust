fn main() {
    imutabilidade();
    mutabilidade();
    shadowing();
}

fn imutabilidade() {
    let x = 5;
    println!("A variável é: {}", x);
    x = 6; //Erro de compilação porque a variável imutável não pode ser reatribuída.
    println!("Mutabilidade sucks, hehe {}", x);
}

fn mutabilidade() {
    let mut x = 5;
    println!("A variável é: {}", x);
    x = 6; //Funciona normalmente porque a keyword mut deixou a variável mutável
    println!("Mutabilidade sucks, hehe {}", x);
}

fn shadowing() {
    let x = 5;
    let x = x + 5;
    let x = x * 2; //Usando shadowing para recriar a variável imutável com o novo valor
    println!("Valor final: {}", x);

    /* 
    Shadowing é diferente do que dizer que uma variável é mut, porque teremos um erro em tempo de compilação se, acidentalmente, tentarmos reatribuir essa variável sem utilizar let. Usando let, nós podemos realizar algumas transformações, mas sem ter uma variável mutável após estas transformações terem sido concluídas.
    Shadowing inclusive permite a mudança de tipo da variável, algo que a variável mutável(com mut) não permite
    */
}