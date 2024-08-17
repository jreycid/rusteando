/*
    rustup: ferramenta de instalación de Rust.

    cargo: ferramenta de compilación e xestión de paquetes de Rust,
            o igual que npm o é de JavaScript.

            Principais comandos:

                cargo --version
                cargo new
                cargo buid
                cargo run
                cargo test
                cargo doc

    crates.io: repositorio central de Rust.

    toml: formato para ficheiros de configuración cama yaml ou json.

    Cargo.toml: describe as dependencias do proxecto e está mantido polo programador.

    cargo.lock: describe as dependencias do proxecto e está mantido por cargo.
                Non se debe editar manualmente, xa que o seu obxectivo é describir
                o estado do proxecto no momento da última compilación exitosa.
                Este arquivo sempre debe estar trackeado por git, xa que é a garantía
                de que distintos desarrolladores en distintos momentos temporales
                podan xerar builds idénticas.

                Para información mais detallada a este respecto, consultar:
                https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
                https://doc.rust-lang.org/cargo/faq.html#why-have-cargolock-in-version-control

    .gitignore: inclúe por defecto a carpeta /target xa que é donde estarán as compilacións.

*/

mod _00_modulos;
mod _01_tipos_de_datos;

fn main() {
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

    _00_modulos::hola_mundo();

    _01_tipos_de_datos::tipos_de_datos();
}
