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
mod modules;

fn main() {

    modules::_00_modulos::hola_mundo();
    modules::_01_variables::variables();
    modules::_02_tipos_de_datos::tipos_de_datos();
    modules::_03_funcions::funcions();
}
