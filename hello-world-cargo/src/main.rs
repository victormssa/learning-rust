fn main() {
    //A linha abaixo imprime "Hello, world!" no console.
    
    println!("Hello, world!");
    
    let mut x: u32 = 713; 
    
    /*u32 = Se refere a um inteiro positivo de 32 bits,
    ou seja, um número inteiro que pode ser representado
    em 32 bits de memória. Esse tipo de dado pode armazenar
    valores inteiros na faixa de 0 até 2.147.483.647. Enquanto
    i32 possui valores entre -2.147.483.648 a 2.147.483.647 */
    
    println!("O valor de X é equivalente a {}", x);
    
    /*É um pouco desconfortável aprender Rust
    de inicio, mas acho que é o costume.*/
    
    x = 60;
    
    println!("O valor novo de X é equivalente a {}", x);
    
    let _float: f32 = 10.5;
    
    let _boolean: bool = false;
    
    if x <= 70 {
        println!("O valor de {} é menor ou igual que 70!", x);
    } else if x >= 70{
        println!("O valor de {} é maior ou igual que 70!", x);
    } else {
        println!("O valor da variavel nem é sequer um número!");
    }
}
