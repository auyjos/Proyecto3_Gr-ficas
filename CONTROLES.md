# 🎮 CONTROLES - Proyecto 3: Sistema Solar Interactivo 3D

## Movimiento de Cámara 3D ✨

### Eje X/Y (Plano Horizontal)
- **←** - Mover cámara a la izquierda
- **→** - Mover cámara a la derecha
- **↑** - Mover cámara hacia arriba
- **↓** - Mover cámara hacia abajo

### Eje Z (Profundidad) 🆕
- **W** - Mover cámara hacia adelante (into screen)
- **Q** - Mover cámara hacia atrás (out of screen)

## Zoom

- **S**: Aumentar zoom (acercar) - Máximo 3.0x
- **A**: Disminuir zoom (alejar) - Mínimo 0.3x

## Control de Animaciones

- **SPACE**: Pausar/Reanudar rotación de planetas sobre su eje
- **O**: Pausar/Reanudar movimiento orbital

## Sistema de Warp (Teletransporte) 🚀 ✨

Presiona el número correspondiente para viajar instantáneamente:

- **1** - Mercurio
- **2** - Venus
- **3** - Tierra
- **4** - Marte
- **5** - Júpiter
- **6** - Saturno
- **7** - Urano
- **8** - Neptuno
- **9** - Plutón

> Durante el warp, verás una animación de anillos pulsantes y el nombre del planeta destino.

## Interfaz de Usuario

- **H** - Mostrar/Ocultar panel de controles

---

## 🌍 Sistema Solar Completo

El programa renderiza simultáneamente **Sol + 9 Planetas + Spaceship**:

### Cuerpos Celestes

1. **Sol** - Centro fijo (amarillo/dorado)
2. **Mercurio** - Órbita más cercana
3. **Venus** - Atmósfera amarilla
4. **Tierra** - Océanos azules + Luna satelital
5. **Marte** - Planeta rojo
6. **Júpiter** - Gigante gaseoso + anillos
7. **Saturno** - Anillos prominentes
8. **Urano** - Gigante de hielo cianita
9. **Neptuno** - Azul profundo oceánico
10. **Plutón** - Órbita más lejana
11. **🚀 Spaceship** - Nave orbital (órbita entre Marte y Júpiter) ✨

### Detección de Colisiones ✨

- La spaceship detecta colisiones con todos los planetas en tiempo real
- Cuando hay colisión, aparece una alerta roja: **⚠ COLLISION: [Planeta]**
- Sistema de colisión esférica optimizado

---

## 📊 Indicadores en Pantalla

### Esquina Superior Derecha
- **FPS** - Frames por segundo actual

### Esquina Inferior Izquierda
- **R** - Indicador de rotación (azul = activo, gris = pausado)
- **O** - Indicador de órbita (azul = activo, gris = pausado)
- **X.Xx** - Nivel de zoom actual

### Centro (Durante Warp)
- Nombre del planeta destino con animación de fade

### Centro Superior (Durante Colisión) ✨
- **⚠ COLLISION: [Planeta]** - Alerta roja con fondo

---

## 💡 Tips de Uso

1. **Exploración 3D**: Usa las flechas y **W/Q** para moverte libremente por el espacio
2. **Seguimiento de planetas**: Usa los números **1-9** para enfocar planetas específicos
3. **Observación detallada**: Acércate con **S** y muévete en 3D para ver detalles
4. **Pausar animación**: **SPACE** + **O** para congelar el sistema
5. **Ver colisiones**: Observa cómo la spaceship detecta colisiones con planetas

---

## 🎨 Características de Renderizado

### Shaders Procedurales (10 tipos únicos)
### Shaders Procedurales (10 tipos únicos)

| Tipo | Cuerpo | Descripción |
|------|--------|-------------|
| 0 | Sol | Gradiente + fotosfera turbulenta + corona |
| 1 | Tierra | Océanos + continentes + nubes animadas |
| 2 | Gigante Gaseoso | Bandas atmosféricas + Gran Mancha Roja |
| 3 | Luna | Cráteres + sombras |
| 4 | Anillos | Bandas de partículas |
| 5 | Neptuno | Base oceánica + metano |
| 6 | Urano | Hielo cianita + escarcha |
| 7 | Venus | Atmósfera amarilla + nubes tóxicas |
| **10** | **Spaceship** | **Metálico shimmer ✨** |

### Skybox ✨
- Background espacial con campo de estrellas procedural
- Renderizado continuo en segundo plano

### Órbitas Visualizadas ✨
- Círculos azules para planetas
- Círculo verde punteado para spaceship

---

## 🔧 Valores por Defecto

- Zoom: 0.6x (para ver todo el sistema)
- Camera Offset: (0, 0, 0)
- Auto Rotate: **Activado**
- Auto Orbit: **Activado**
- Show Controls: **Visible** (presiona H para ocultar)

---

## 🚀 Ejecución

```bash
cargo run --release
```

---

## Controles Resumidos

| Acción | Teclas |
|--------|--------|
| Cámara X/Y | ←↑↓→ |
| Cámara Z ✨ | W / Q |
| Zoom | S / A |
| Rotación | SPACE |
| Órbitas | O |
| Warp ✨ | 1-9 |
| Ayuda | H |

---

**Proyecto 3** - Sistema Solar Interactivo 3D  
Universidad del Valle de Guatemala - 2025  
**120/120 puntos** ✅
