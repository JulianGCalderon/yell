# Yell

Interfaz desarrollada en Rust con GTK4 que busca y descarga videos de youtube. Para hacerlo, necesitará de una clave de su API.

![demo](https://github.com/JulianGCalderon/youtube-downloader/assets/60768809/ae7f2c12-f3f7-46bb-8b07-c74a4e39ec64)

La aplicación permite buscar videos de youtube y visuializar los resultados en una lista. Si hay espacio suficiente, se mostrará información del video seleccionado a la derecha. No se puede previsualizar el video, únicamente se podra ver la miniatura del mismo.

## Ejecución

Para ejecutarlo, es necesario tener instalado `cargo`, `GTK4` versión 4.6 o superior, y `libadwaita` versión 1.1 o superior.

```bash
$ cargo run
```

## Clave de API

Para funcionar, necesita de una clave de API de YouTube que se proporciona a través de una variable de entorno de nombre `API_KEY`. También se puede agregar un archivo `.env` en el directorio raiz del proyecto que contenga dicha clave.
