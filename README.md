# Fox Hell - Rust

## Developed and Installation

El lenguaje de programación que se utilizó para este proyecto es **Rust**, desarrollado por Mozilla y diseñado para ser rápido, eficiente y seguro.

Como tecnología auxiliar se usó **WebAssembly**, la cual nos ofrece una compilación de código **Rust** a un binario `.wasm` que mejora la eficiencia de las aplicaciones web con una velocidad comparable a la de una aplicación de escritorio.

## Instalación

Para poder llevar a cabo la ejecución de este proyecto se deben seguir los siguientes pasos:

#### 1. Lenguaje Rust (nightly)

Deberemos descargar e instalar el lenguaje **Rust** en su versión *nightly*, esto lo podemos hacer desde la [página oficial](https://www.rust-lang.org/tools/install "Instalación").

Cuando nos encontremos en la aplicación de instalación deberemos personalizarla cambiando únicamente la versión, de *stable* a *nightly*, una vez hecho esto continuamos con la instalación normalmente.

#### 2. Compilador WASM

Para poder compilar nuestro código Rust a código **WebAssembly** necesitamos instalar esa opción de compilación, para ello usaremos `rustup` ejecutando:

	rustup target add wasm32-unknown-unknown

#### 3. Servidor

Por último necesitaremos instalar un servidor de WebAssembly con el fin de poder depurar nuestro código mientras lo vamos escribiendo o modificando. El administrador de paquetes de **Rust**, `cargo`, nos permite esto con sólo un par de comandos:

	cargo +nightly install wasm-bindgen-cli
	cargo install -f cargo-web

#### 4. Finalización y comprobación

Para comprobar que nuestro entorno ya está preparado para correr este tipo de aplicaciones debemos descargar este repositorio y en el directorio raíz ejecutar:

	cargo web start --target=wasm32-unknown-unknown

Esto montará nuestro servidor con la aplicación ejecutándose en nuestro `localhost` y recompilando cada que se detecte un cambio. Para notar los cambios en el navegador es necesario refrescar la página.

## Descripción del proyecto

Se desarrolló un mini juego escrito en el lenguaje **Rust** pero con las capacidades gráficas de web con **WebAssembly**.

El juego consiste en un zorro que debe recolectar manzanas mientras esquiva las rocas de lava, se mostrará la puntuación en la parte superior derecha de la pantalla, siendo un punto por manzana. Al mismo tiempo se tienen las vidas mostradas como corazones en la parte superior izquierda.

Durante la ejecución del juego se tiene a un lado una representación de la máquina de estados que indican el estado del jugador, así como las transiciones de estados del mismo.
