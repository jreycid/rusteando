/*
    Tipado estático e forte

    Se non se indica o tipo da vble, o compilador infireo do valor otorgado.

    Tipos primitivos:
        Integer
        Floating-point
        Boolean
        Character

    
*/

pub fn tipos_de_datos() {
    println!("\nTipos de datos en Rust:");

    /*
        Integer:
            u para naturais
            i para enteiros
            bits: de 8 a 128
    */
    let a: u32 = 100;
    let b: i32 = 100;

    println!("número natural: {}", a);
    println!("número enteiro: {}", b);

    /*
        Float:
            f
            bits: 32 ou 64
    */
    let c: f32 = 100000.0;
    println!("número real: {}", c);

    /*
        Boolean
    */
    let flag: bool = true;
    println!("flag é un boolean con valor: {}", flag);
}