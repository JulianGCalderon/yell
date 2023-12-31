# Youtube Downloader

Interfaz desarrollada en Rust con GTK4 que busca y descarga videos de youtube. Para hacerlo, necesitará de una clave de su API.

Fue inicialmente desarrollado en Python, pero decidi migrarlo a Rust.

## Ejecución

Para ejecutarlo, es necesario tener instalado `cargo` y `GTK4`, versión 4.6 o superior.

```bash
$ cargo run
```

## Clave de API

Para funcionar, necesita de una clave de API de YouTube que se proporciona a través de una variable de entorno de nombre `API_KEY`. También se puede agregar un archivo `.env` en el directorio raiz del proyecto que contenga dicha clave.
