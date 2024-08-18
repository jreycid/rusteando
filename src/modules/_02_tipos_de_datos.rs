/*
    Tipado estático e forte

    Se non se indica o tipo da vble, o compilador infireo do valor otorgado.

    Tipos primitivos:
        Integer
        Floating-point
        Boolean
        Character

    Coleccións de datos:
        Tuples
        Structs
        Enums
*/

pub fn tipos_de_datos() {
    println!("\nTipos de datos en Rust:");

    /*
        Inferencia de tipos de datos:

        Cando declaramos unha vble, se
    */

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

    /*
        Tuplas:

            Aos valores individuales dunha tupla chámaselles elementos
            Os elementos da tupla poden ser de distintos tipos
            As tuplas teñen lonxitude fixa unha vez declaradas
            Non se pode engadir nin eliminar eltos dunha tupla
            O acceso os elementos faise co operador '.'
            Un uso típico das tuplas e no return dunha función

    */
    let tupla = ("cadea", 's', 1.0, 2, false);
    println!(
        "Contido da tupla: {} {} {} {} {}",
        tupla.0, tupla.1, tupla.2, tupla.3, tupla.4
    );

    /*
        Structs:

            Similar os obxectos doutras linguaxes ou os structs de C e C++
            Aos valores individuales dun struct chámaselles campos
            Os campos dunha struct poden ser de distintos tipos
            A cada campo pódeselle dar un nome
            Para usar un struct, defínese o tipo de dato de cada campo e
            dáselle un nome o tipo de esturctura definido, despois hai que instancialo.
            Os structs non poden almacenar strings de tipo literal (&str)

            Rust define 3 tipos de structs:

                Classic C structs
                Tuple structs: a diferenza dos classic, os campos non teñen nome
                Unit structs:

    */
    // Classic C struct
    struct Persona {
        nome: String,
        apelido: String,
        idade: u8,
    }
    let persona = Persona {
        nome: "Jose".to_string(),
        apelido: String::from("Rey"),
        idade: 40,
    };
    println!("Classic C struct:");
    println!(
        "nome: {}, apelido: {}, idade: {} anos",
        persona.nome, persona.apelido, persona.idade
    );

    // Tuple structs
    struct Cor(u32, u32, u32);
    let branco = Cor(255, 255, 255);
    println!("Tuple struct:");
    println!("Cor: {}, {}, {}", branco.0, branco.1, branco.2);

    /*
        Enums

            Cada valor dun enum chámase variante
            Para acceder aos valores dunha enumeración, usamos o operador ::
            Poden conter struts
    */
    // #[allow(dead_code)] -> para que o compilador non se queixe se non
    //                   empregamos todos os campos das nosas estruturas
    #[allow(dead_code)]
    // #[derive(Debug)] -> emprégase en combinación con {:?} para amosar 
    //                  o contido dunha estrutura enum de forma lexible
    #[derive(Debug)]
    enum Dia {
        Luns,
        Martes,
        Mércores,
        Xoves,
        Venres,
        Sábado,
        Domingo,
    }
    let dia = Dia::Domingo;
    println!("Hoxe é {:?}", dia);

}
