# Lab 2 - Conway's Game of Life

Laboratorio de Graficas por Computadora. Implementa el Juego de la Vida de
Conway en tiempo real dentro de una ventana de raylib, pintando cada celula
unicamente con `point()` (sin `draw_rectangle` ni otras primitivas), sobre
un framebuffer interno de celulas que puede tener menor resolucion que la
ventana.

## Estructura del proyecto

```
src/
  framebuffer.rs   struct Framebuffer: grilla de celulas (Vec<bool>),
                   wrap toroidal, get_color(), step() (reglas de Conway
                   con doble buffer) y render() (dibuja todo con point())
  organisms.rs     una funcion por cada organismo clasico (glider,
                   naves, osciladores, still lifes)
  main.rs          configuracion de ventana/grilla, arma el patron
                   inicial repartiendo los organismos por el tablero
                   y corre el loop de simulacion en tiempo real
```

## Reglas de Conway

Implementadas tal cual en `Framebuffer::step` (`src/framebuffer.rs`):

1. Una celula viva con menos de dos vecinos vivos muere.
2. Una celula viva con dos o tres vecinos vivos sobrevive.
3. Una celula viva con mas de tres vecinos vivos muere.
4. Una celula muerta con exactamente tres vecinos vivos nace.

`step()` calcula la generacion siguiente completa en un vector nuevo antes
de reemplazar el estado actual, para no contar vecinos sobre celulas que ya
cambiaron en la misma pasada. El framebuffer nunca se limpia entre frames:
cada celda de la siguiente generacion sale de las reglas aplicadas sobre el
estado anterior completo, no de un tablero vacio.

## Bordes toroidales

`Framebuffer::index` convierte cualquier `(x, y)` (incluyendo negativos o
mayores al tamano de la grilla) a un indice valido con `rem_euclid`. Esto
centraliza el wrap-around: si una celula sale por la derecha, el modulo la
hace aparecer por la izquierda, y lo mismo verticalmente. `get`, `set` y
`count_neighbors` usan siempre este mismo camino, asi que el wrap aplica en
toda la simulacion sin casos especiales.

## Organismos incluidos

`organisms.rs` define block, loaf, blinker, toad, beacon, pulsar, glider,
lightweight/middleweight/heavyweight spaceship. Cada uno recibe la posicion
donde se coloca su esquina superior izquierda. `main.rs` reutiliza estas
funciones varias veces para repartir el patron inicial en tres franjas del
tablero.

## Requisitos

- Rust + Cargo (edicion 2021 o superior).
- Un compilador de C y CMake instalados y accesibles desde la terminal
  (raylib-sys compila raylib desde codigo fuente en el primer build). En
  Windows, la forma mas simple es tener MSYS2/MinGW-w64 con `gcc` y `cmake`
  en el PATH.

**Nota (Windows/MSYS2):** si `cargo build` falla con errores de
`CMakeTestCCompiler` o `cc1.exe` sin poder compilar un programa de prueba,
suele ser un PATH que mezcla `mingw64\bin` y `ucrt64\bin` (DLLs de dos
toolchains distintas pisandose). Asegurate de tener solo una de las dos
carpetas de MSYS2 en el PATH, con esa carpeta antes que cualquier otra
entrada de MinGW.

## Como ejecutar

```
cargo run
```

Abre una ventana y corre la simulacion en tiempo real: cada frame calcula
la siguiente generacion (`fb.step()`) y la dibuja (`fb.render()`), con un
pequeno delay para poder ver la evolucion. Cerrar la ventana termina el
programa.

## Configuracion (src/main.rs)

```rust
const GRID_WIDTH: i32 = 100;   // celulas de ancho del framebuffer
const GRID_HEIGHT: i32 = 100;  // celulas de alto del framebuffer
const CELL_SIZE: i32 = 8;      // pixeles de ventana por celula
const DELAY_MS: u64 = 100;     // delay entre frames, en milisegundos
```

La ventana siempre mide `GRID_WIDTH * CELL_SIZE` por `GRID_HEIGHT *
CELL_SIZE`. Bajar `CELL_SIZE` mete mas celulas en la misma ventana; subirlo
hace las celulas mas grandes y faciles de ver.

### Configuraciones sugeridas para probar

| GRID_WIDTH/HEIGHT | CELL_SIZE | Ventana resultante | Que observar |
|---|---|---|---|
| 100x100 | 8 | 800x800 | celulas grandes, facil ver cada organismo, muy fluido |
| 200x200 | 4 | 800x800 | el doble de celulas, sigue fluido, celulas medianas |
| 300x300 | 3 | 900x900 | 90000 celulas por `step()`, buen caso para medir si baja el framerate |

`DELAY_MS` tambien se puede bajar (ej. 30-50) para ver la simulacion mas
rapido, sobre todo util al probar 300x300.

Las posiciones de los organismos en `main.rs` estan pensadas para una
grilla de 100x100; si subes a 200x200 o 300x300 el patron va a quedar
concentrado en una esquina del tablero (el resto queda vacio, lo cual
sigue siendo un resultado valido). Para que ocupe todo el tablero a otra
resolucion habria que escalar esas coordenadas proporcionalmente.

## Como generar un GIF para el README / Discord

1. Graba la ventana de la simulacion corriendo unos 5-10 segundos con
   alguna herramienta de captura:
   - **ScreenToGif** (Windows, gratis): graba directo a `.gif`, ya trae
     editor para recortar y ajustar el framerate antes de exportar. Es la
     opcion mas simple porque no requiere pasos adicionales.
   - Alternativa: grabar con el Xbox Game Bar (`Win+G`) o OBS a `.mp4`, y
     convertir con `ffmpeg` (metodo de paleta, dos pasadas, para que el
     gif no pese demasiado):
     ```
     ffmpeg -i grabacion.mp4 -vf "fps=15,scale=480:-1:flags=lanczos,palettegen" paleta.png
     ffmpeg -i grabacion.mp4 -i paleta.png -filter_complex "fps=15,scale=480:-1:flags=lanczos[x];[x][1:v]paletteuse" demo.gif
     ```
2. Guarda el resultado como `demo.gif` en la raiz del proyecto y enlazalo
   en este README:
   ```
   ![demo](demo.gif)
   ```
3. Para subirlo a Discord, arrastra el archivo `.gif` directo al chat.
   Discord tiene limite de tamano por archivo segun el servidor (8 MB en
   servidores normales, mas en los con boost); si el gif pesa de mas,
   baja el `fps` a 10-12, el `scale` a 320-360, o recorta la duracion.

## Pruebas sugeridas

- Confirmar que el block y el loaf (still lifes) no cambian de forma en
  ningun frame.
- Confirmar que el blinker, el toad y el beacon oscilan entre dos formas
  cada 2 generaciones, y el pulsar vuelve a su forma cada 3 generaciones.
- Seguir con la vista un glider o una nave (LWSS/MWSS/HWSS) varios
  segundos y confirmar que se desplaza en linea recta, reapareciendo del
  otro lado del tablero al cruzar un borde (wrap toroidal).
- Cambiar `GRID_WIDTH`/`GRID_HEIGHT`/`CELL_SIZE` a las configuraciones de
  la tabla de arriba y confirmar que la ventana sigue viendose correcta
  (celulas escaladas, sin huecos ni artefactos) a cada resolucion.
