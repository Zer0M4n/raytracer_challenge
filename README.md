# Ray Tracer Challenge

## English

This project is still under development and is in an early stage.

The goal of this project is to build a ray tracer in Rust without using external libraries, only the Rust standard library.

It is based on the book "The Ray Tracer Challenge" by Jamis Buck and other sources. For more information about how it was built, see [LEARN.md](LEARN.md).

### Run the project

Make sure Rust and Cargo are installed.

```bash
cargo build
cargo run
```

### Run the tests

```bash
cargo test
```

## Español

Este proyecto sigue en desarrollo y se encuentra en una etapa temprana.

El objetivo de este proyecto es construir un ray tracer en Rust sin usar librerías externas, solo la librería estándar de Rust.

Está basado en el libro "The Ray Tracer Challenge" de Jamis Buck y otras fuentes. Para saber más sobre cómo fue construido, consulta [LEARN.es.md](LEARN.es.md).

### Ejecutar el proyecto

Asegúrate de tener Rust y Cargo instalados.

```bash
cargo build
cargo run
```

### Ejecutar los tests

```bash
cargo test
```

## Last IMAGE generated

![Last image generated](img/first_image_with_reflective.jpg)

## Gallery

<table>
  <tr>
    <td align="center">
      <img src="img/first_circle.jpg" width="250"/><br/>
      <sub>First circle</sub>
    </td>
    <td align="center">
      <img src="img/first_circle_with_light.jpg" width="250"/><br/>
      <sub>First circle with light</sub>
    </td>
    <td align="center">
      <img src="img/first_image_shadow.jpg" width="250"/><br/>
      <sub>First image with shadow</sub>
    </td>
  </tr>
  <tr>
    <td align="center">
      <img src="img/first_image_with_pattern.jpg" width="250"/><br/>
      <sub>Pattern</sub>
    </td>
    <td align="center">
      <img src="img/first_image_with_plane.jpg" width="250"/><br/>
      <sub>Plane</sub>
    </td>
    <td align="center">
      <img src="img/first_image_with_reflective.jpg" width="250"/><br/>
      <sub>Reflective surfaces</sub>
    </td>
  </tr>
  <tr>
    <td align="center">
      <img src="img/first_plane_the_world.jpg" width="250"/><br/>
      <sub>Full scene: floor, walls & spheres</sub>
    </td>
    <td align="center">
      <img src="img/fuction_image_example.jpg" width="250"/><br/>
      <sub>Function image example</sub>
    </td>
    <td align="center">
      <img src="img/image_example.jpg" width="250"/><br/>
      <sub>Image example</sub>
    </td>
  </tr>
</table>

## Roadmap

Features planned as I work through the rest of the book:

- [ ] **Cubes**
  - [ ] Ray-cube intersection
  - [ ] Cube normals
- [ ] **Cylinders**
  - [ ] Ray-cylinder intersection
  - [ ] Cylinder normals
  - [ ] Truncated cylinders
  - [ ] Capped cylinders
  - [ ] Cones
- [ ] **Groups**
  - [ ] Group implementation
  - [ ] Normals on child objects
  - [ ] Bounding boxes for scene optimization
- [ ] **Triangles**
  - [ ] Triangle intersection
  - [ ] Wavefront OBJ file parsing
  - [ ] Smooth triangles
  - [ ] Smooth triangles from OBJ files
- [ ] **Constructive Solid Geometry (CSG)**
  - [ ] CSG implementation
  - [ ] Coloring CSG shapes
- [ ] **Next steps**
  - [ ] Area lights & soft shadows
  - [ ] Spotlights
  - [ ] Focal blur
  - [ ] Motion blur
  - [ ] Anti-aliasing
  - [ ] Texture maps
  - [ ] Normal perturbation
  - [ ] Torus primitive