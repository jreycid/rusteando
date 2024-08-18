pub fn variables() {
    /*
        En Rust as vbles son, por defecto, inmutables!

            let str = "Mundo";
            str = "Galcia"; -> casca

        Para poder mutar unha vble hai que usar let mut
    */
    let mut str1 = "Mundo";

    println!("\nOla!");
    println!("Ola, {}!", str1);
    str1 = "Galicia";
    println!("Ola, {}!", str1);

    /*
        Se non queremos declarar unha variable como mutable, tamén e válido
        declarar unha nova variable co nome nunha xa existente
        Isto chámase variable shadowing
    */

    let str2 = "Jose";
    println!("Ola, {}!", str2);
    let str2 = str2.to_string() + " Rey";
    println!("Ola, {}!", str2);
}
