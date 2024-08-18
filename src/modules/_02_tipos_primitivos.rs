/*
    Tipado estático e forte

    Se non se indica o tipo da vble, o compilador infireo do valor otorgado.

    Tipos primitivos:
        Integer
        Floating-point
        Boolean
        Character
*/

pub fn tipos_primitivos() {
    println!("\nTipos de datos primitivos en Rust:");

    /*
        Integer:
            u para naturais
            i para enteiros
            bits: de 8 a 128
    */
    let a: u32 = 100;
    let b: i32 = -100;
    println!("número natural: {}", a);
    println!("número enteiro: {}", b);

    /*
        Float:
            f
            bits: 32 ou 64 (por defecto 64)

    */
    let c: f32 = 100000.0;
    println!("número real: {}", c);

    /*
        Boolean
    */
    let flag: bool = true;
    println!("flag é un boolean con valor: {}", flag);

    /*
        Caracteres:

            Admite caracteres únicos e defínense con ''
    */
    let char = 'c';
    let emoji = '🦀';
    println!("char: {}", char);
    println!("emoji: {}", emoji);

    /*
        Cadeas en Rust (similitudes con C e C++):

            Defínense con ""
            Existen dous tipos de strings:

                &str:
                    Inmutable (logo temos que conocer o seu valor en tempo de compilación)
                    De lonxitude fixa
                    Tipo por defecto cando o compilador infiere un string
                    É unha referencia a un segmento de memoria estática (pila) que contén texto

                String:
                    Mutable
                    De lonxitude vble
                    É un array de caracteres almacenados na memoria dinámica (heap).
    */
    let str_inmutable = "Isto é un string inmutable";
    println!("{}", str_inmutable);

    let mut str_mutable = String::new();
    str_mutable.push_str("Isto é un string ");
    str_mutable.push('m');
    str_mutable.push('u');
    str_mutable.push('t');
    str_mutable.push('a');
    str_mutable.push('b');
    str_mutable.push('l');
    str_mutable.push('e');
    println!("{}", str_mutable);

}
