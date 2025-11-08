# Proyecto 3 - Sistema Solar Interactivo 3D

**Renderer 3D de Sistema Solar Completo** - Implementación de un sistema solar con 9 planetas, Sol, Spaceship orbital, Skybox, Sistema de Warp y Detección de Colisiones.

## 🌍 Descripción

Un renderer 3D que renderiza **simultáneamente el Sol y 9 planetas** en órbita, todo con movimiento en tiempo real, texturas procedurales, una nave espacial orbitando, sistema de teletransporte warp, y detección de colisiones. Incluye cámara 3D completamente funcional con controles de zoom y movimiento en los tres ejes.

---

## ✨ Características Principales

### ✅ Requisitos Obligatorios Implementados (120 puntos)

1. **Sol + 9 Planetas Alineados al Plano Eclíptico (10 pts)** ✅
   - Sol (centro fijo)
   - Mercurio, Venus, Tierra, Marte, Júpiter, Saturno, Urano, Neptuno, Plutón
   - Todos en el plano Z=0 (eclíptico)

2. **Órbitas Circulares y Rotación (10 pts)** ✅
   - Cada planeta orbita alrededor del Sol con velocidad única
   - Rotación sobre su propio eje
   - Toggle con SPACE (rotación) y O (órbitas)

3. **Skybox con Estrellas (10 pts)** ✅
   - Background espacial con campo de estrellas procedural
   - Implementado en `src/skybox.rs`

4. **Spaceship Orbital (10 pts)** ✅
   - Nave espacial 3D cargada desde modelo OBJ
   - Orbita en el plano eclíptico (entre Marte y Júpiter)
   - Shader metálico personalizado
   - Radio orbital: 180 unidades
   - Velocidad: 0.15 rad/s

5. **Sistema de Warp (20 pts)** ✅
   - Teclas 1-9 para viajar instantáneamente a cada planeta
   - Animación de anillos pulsantes durante el warp
   - Transición suave de cámara
   - Indicador visual del planeta destino

6. **Zoom (5 pts)** ✅
   - Teclas S (acercar) / A (alejar)
   - Rango: 0.3x - 3.0x
   - Afecta escala de todos los objetos uniformemente

7. **Visualización de Órbitas (5 pts)** ✅
   - Círculos azules para órbitas planetarias
   - Círculo verde punteado para órbita de spaceship
   - Renderizado en tiempo real

8. **Movimiento de Cámara 3D (40 pts)** ✅
   - **Eje X/Y**: Flechas (←↑↓→)
   - **Eje Z**: W (adelante) / Q (atrás)
   - Movimiento suave en los 3 ejes espaciales
   - Offset de cámara aplicado a todos los objetos

9. **Detección de Colisiones (10 pts)** ✅
   - Sistema de colisión esférica entre spaceship y planetas
   - Algoritmo de distancia euclidiana optimizado
   - Alerta visual con nombre del planeta al colisionar
   - Implementado en `src/collision.rs`

---

## 📋 Estructura del Sistema Solar

```
                    ☆ Sol (Centro)
                   /|\
                  / | \
                 /  |  \
                /   |   \
       ◉ Mercurio  ◉ Venus  ◉ Tierra (+ Luna)
      
    ◉ Marte    ◉ Júpiter (+ Anillos)    ◉ Saturno (+ Anillos)
    
        ◉ Urano     ◉ Neptuno     ◉ Plutón
        
            🚀 Spaceship (Órbita entre Marte-Júpiter)
```

**Radios Orbitales:**
- Mercurio: 40 unidades
- Venus: 65 unidades
- Tierra: 90 unidades (+Luna satélite)
- Marte: 120 unidades
- **Spaceship: 180 unidades** 🚀
- Júpiter: 140 unidades (+ anillos)
- Saturno: 175 unidades (+ anillos)
- Urano: 160 unidades
- Neptuno: 180 unidades
- Plutón: 200 unidades

---

## 🎮 Controles Completos

### Movimiento de Cámara (3D)
- **←→** - Mover cámara en eje X (izquierda/derecha)
- **↑↓** - Mover cámara en eje Y (arriba/abajo)
- **W** - Mover cámara hacia adelante (eje Z+)
- **Q** - Mover cámara hacia atrás (eje Z-)

### Zoom
- **S** - Acercar zoom (hasta 3.0x)
- **A** - Alejar zoom (hasta 0.3x)

### Animaciones
- **SPACE** - Pausar/Reanudar rotación de planetas
- **O** - Pausar/Reanudar órbitas

### Sistema Warp
- **1** - Warp a Mercurio
- **2** - Warp a Venus
- **3** - Warp a Tierra
- **4** - Warp a Marte
- **5** - Warp a Júpiter
- **6** - Warp a Saturno
- **7** - Warp a Urano
- **8** - Warp a Neptuno
- **9** - Warp a Plutón

### UI
- **H** - Mostrar/Ocultar panel de controles

---

## 📁 Estructura del Proyecto

```
Proyecto3/
├── Cargo.toml                    # Dependencias Rust
├── README.md                     # Este documento
├── CONTROLES.md                  # Referencia rápida de controles
├── src/
│   ├── main.rs                  # Loop principal + render pipeline
│   ├── collision.rs             # ✨ Sistema de detección de colisiones
│   ├── spaceship.rs             # ✨ Spaceship orbital
│   ├── warp.rs                  # ✨ Sistema de teletransporte
│   ├── skybox.rs                # ✨ Background de estrellas
│   ├── orbits.rs                # Renderizado de órbitas
│   ├── solar_system.rs          # Configuración de planetas
│   ├── shaders.rs               # Shaders procedurales (10 tipos)
│   ├── framebuffer.rs           # Gestión de píxeles
│   ├── triangle.rs              # Rasterización de triángulos
│   ├── obj.rs                   # Cargador de modelos OBJ
│   ├── vertex.rs                # Estructura de vértices
│   ├── fragment.rs              # Fragment shader
│   ├── matrix.rs                # Matemáticas de matrices 4x4
│   ├── texture.rs               # Sistema de texturas
│   ├── rings.rs                 # Generación de anillos planetarios
│   └── moons.rs                 # Sistema de lunas
├── assets/
│   └── models/
│       ├── spaceship/           # ✨ Modelo de nave espacial
│       │   ├── SpaceShip.obj
│       │   └── SpaceShip.mtl
│       ├── 13902_Earth_v1_l3.obj
│       ├── 13905_Jupiter_V1_l3.obj
│       ├── 13907_Uranus_v2_l3.obj
│       ├── 13913_Sun_v2_l3.obj
│       └── ...
└── target/                      # Binarios compilados
```

---

## 🚀 Instalación y Ejecución

### Prerrequisitos

- **Rust**: 1.70 o superior ([Instalar Rust](https://rustup.rs/))
- **Cargo**: Viene con Rust
- **Compilador C++**: Requerido por Raylib
  - **Linux**: `sudo apt-get install build-essential`
  - **macOS**: `xcode-select --install`

### Compilar y Ejecutar

```bash
# Clonar repositorio
git clone https://github.com/auyjos/Proyecto3_Gr-ficas.git
cd Proyecto3_Gr-ficas

# Cambiar a rama correcta
git checkout feature/alineacion

# Compilar en modo release (optimizado)
cargo build --release

# Ejecutar
cargo run --release
```

---

## 🧮 Detalles Técnicos

### Pipeline de Renderizado

1. **Vertex Processing**
   - Carga de vértices desde OBJ
   - Transformaciones model-view-projection
   - Aplicación de offset de cámara 3D

2. **Rasterization**
   - Conversión de triángulos a píxeles
   - Coordenadas baricéntricas
   - Interpolación de profundidad

3. **Fragment Processing**
   - 10 shaders procedurales únicos
   - Efectos temporales (animación)
   - Escritura a framebuffer

4. **Collision Detection**
   - Algoritmo de esferas (sphere-sphere)
   - Cálculo de distancia euclidiana optimizado
   - Detección en tiempo real

### Shaders Implementados

| ID | Tipo | Descripción |
|----|------|-------------|
| 0 | Sol | Gradiente de núcleo + fotosfera turbulenta |
| 1 | Tierra | Océanos + continentes + nubes animadas |
| 2 | Gigante Gaseoso | Bandas atmosféricas + Gran Mancha Roja |
| 3 | Luna | Cráteres + sombras + picos brillantes |
| 4 | Anillos | Bandas de partículas + profundidad |
| 5 | Neptuno | Base oceánica + metano + Gran Mancha Oscura |
| 6 | Urano | Hielo cianita + patrones de escarcha |
| 7 | Venus | Atmósfera amarilla + nubes tóxicas |
| 8-9 | Reservados | - |
| **10** | **Spaceship** | **Metálico shimmer + reflejo dinámico** ✨ |

### Sistema de Colisiones

```rust
// Algoritmo de detección (src/collision.rs)
pub fn check_sphere_collision(
    pos1: Vector3, radius1: f32,
    pos2: Vector3, radius2: f32
) -> bool {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    let dz = pos1.z - pos2.z;
    let distance_squared = dx*dx + dy*dy + dz*dz;
    distance_squared < (radius1 + radius2)²
}
```

---

## 📦 Dependencias

| Crate | Versión | Propósito |
|-------|---------|-----------|
| **raylib** | 5.5.1 | Window management + input |
| **tobj** | 4.0.2 | OBJ file parsing |

---

## 🎯 Puntuación del Proyecto

| Requisito | Puntos | Estado |
|-----------|--------|--------|
| Sol + 9 planetas alineados | 10 | ✅ |
| Órbitas y rotaciones | 10 | ✅ |
| Skybox | 10 | ✅ |
| Spaceship | 10 | ✅ |
| Sistema warp | 20 | ✅ |
| Zoom | 5 | ✅ |
| Órbitas visualizadas | 5 | ✅ |
| Cámara 3D (X/Y/Z) | 40 | ✅ |
| Collision Detection | 10 | ✅ |
| **TOTAL** | **120** | **✅ 120/120** |

---

## 🎓 Conceptos Demostrados

- Matemáticas de matrices 4x4
- Transformaciones afines (traslación, rotación, escalado)
- Pipeline gráfico completo
- Rasterización de triángulos
- Shaders procedurales multi-capa
- Mecánica orbital realista
- Detección de colisiones 3D
- Sistemas de partículas (skybox)
- Gestión de framebuffer
- Interpolación baricéntrica

---

## 📹 Video Demostrativo

[Pendiente de subir]

---

## 👨‍💻 Autor

**Jose Rodrigo Barrera García**  
Universidad del Valle de Guatemala  
Gráficas por Computadora - 2025

---

## 📝 Licencia

Este proyecto es parte de un curso de gráficas por computadora con fines educativos.

---

**Última Actualización**: 7 de noviembre, 2025  
**Versión del Proyecto**: 3.0 (Final)
