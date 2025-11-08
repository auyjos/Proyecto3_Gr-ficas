/// Tipos de cuerpos celestes con shaders específicos
#[derive(Clone, Copy, Debug)]
pub enum CelestialType {
    Sun = 0,
    RockyPlanet = 1,
    GasGiant = 2,
    #[allow(dead_code)]
    Moon = 3,
    #[allow(dead_code)]
    Ring = 4,
    IceGiant = 5,
    Uranus = 6,
    Venus = 7,
}

/// Definición de un cuerpo celeste en el sistema solar
#[derive(Clone)]
pub struct CelestialBody {
    pub name: String,
    pub celestial_type: CelestialType,
    pub scale: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub rotation_speed: f32,
    pub model_path: String,
    pub has_rings: bool,
    pub moons: Vec<Moon>,
}

/// Definición de una luna
#[derive(Clone)]
pub struct Moon {
    #[allow(dead_code)]
    pub name: String,
    pub scale: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub rotation_speed: f32,
}

impl CelestialBody {
    pub fn new(
        name: &str,
        celestial_type: CelestialType,
        scale: f32,
        orbit_radius: f32,
        orbit_speed: f32,
        rotation_speed: f32,
        model_path: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            celestial_type,
            scale,
            orbit_radius,
            orbit_speed,
            rotation_speed,
            model_path: model_path.to_string(),
            has_rings: false,
            moons: Vec::new(),
        }
    }

    pub fn with_rings(mut self, has_rings: bool) -> Self {
        self.has_rings = has_rings;
        self
    }

    pub fn with_moon(mut self, moon: Moon) -> Self {
        self.moons.push(moon);
        self
    }

    pub fn planet_type(&self) -> u32 {
        self.celestial_type as u32
    }
}

impl Moon {
    pub fn new(
        name: &str,
        scale: f32,
        orbit_radius: f32,
        orbit_speed: f32,
        rotation_speed: f32,
    ) -> Self {
        Self {
            name: name.to_string(),
            scale,
            orbit_radius,
            orbit_speed,
            rotation_speed,
        }
    }
}

/// Configuración del sistema solar con todos los planetas en orden correcto
pub fn create_solar_system() -> Vec<CelestialBody> {
    vec![
        // 1. Sol (centro del sistema)
        CelestialBody::new(
            "Sol",
            CelestialType::Sun,
            50.0,
            0.0,
            0.0,
            0.02,
            "assets/models/13913_Sun_v2_l3.obj",
        ),
        
        // 2. Mercurio (primer planeta, más cercano al sol)
        CelestialBody::new(
            "Mercurio",
            CelestialType::RockyPlanet,
            12.0,
            80.0,
            0.40,  // Órbita más rápida
            0.015,
            "assets/models/mercury.obj",
        ),
        
        // 3. Venus (segundo planeta)
        CelestialBody::new(
            "Venus",
            CelestialType::Venus,
            18.0,
            120.0,
            0.30,
            0.008,
            "assets/models/venus.obj",
        ),
        
        // 4. Tierra con Luna (tercer planeta)
        CelestialBody::new(
            "Tierra",
            CelestialType::RockyPlanet,
            20.0,
            180.0,
            0.20,
            0.03,
            "assets/models/13902_Earth_v1_l3.obj",
        )
        .with_moon(Moon::new("Luna", 8.0, 50.0, 0.08, 0.05)),
        
        // 5. Marte (cuarto planeta - planeta rojo)
        CelestialBody::new(
            "Marte",
            CelestialType::RockyPlanet,
            17.0,
            240.0,
            0.15,
            0.03,
            "assets/models/10464_Asteroid_v1_Iterations-2.obj", // Textura rojiza del asteroide
        ),
        
        // 6. Júpiter (quinto planeta - gigante gaseoso más grande)
        CelestialBody::new(
            "Júpiter",
            CelestialType::GasGiant,
            40.0,
            320.0,
            0.10,
            0.045,
            "assets/models/13905_Jupiter_V1_l3.obj",
        )
        .with_rings(false), // Júpiter tiene anillos débiles, pero no visibles a esta escala
        
        // 7. Saturno (sexto planeta - con anillos prominentes)
        CelestialBody::new(
            "Saturno",
            CelestialType::GasGiant,
            35.0,
            400.0,
            0.08,
            0.040,
            "assets/models/13905_Jupiter_V1_l3.obj", // Reutilizar Júpiter con shader diferente
        )
        .with_rings(true), // Anillos icónicos de Saturno
        
        // 8. Urano (séptimo planeta - gigante de hielo)
        CelestialBody::new(
            "Urano",
            CelestialType::Uranus,
            26.0,
            480.0,
            0.06,
            0.035,
            "assets/models/Uranus.obj",
        )
        .with_rings(true), // Urano tiene anillos sutiles
        
        // 9. Neptuno (octavo planeta - gigante de hielo azul)
        CelestialBody::new(
            "Neptuno",
            CelestialType::IceGiant,
            25.0,
            560.0,
            0.04,
            0.032,
            "assets/models/Neptune.obj",
        ),
    ]
}

/// Devuelve una lista de rutas únicas de modelos a cargar
pub fn get_unique_model_paths() -> Vec<&'static str> {
    vec![
        "assets/models/13913_Sun_v2_l3.obj",          // Sol
        "assets/models/mercury.obj",                   // Mercurio
        "assets/models/venus.obj",                     // Venus
        "assets/models/13902_Earth_v1_l3.obj",        // Tierra
        "assets/models/10464_Asteroid_v1_Iterations-2.obj", // Marte
        "assets/models/13905_Jupiter_V1_l3.obj",      // Júpiter y Saturno
        "assets/models/Uranus.obj",                    // Urano
        "assets/models/Neptune.obj",                   // Neptuno
        "assets/models/moon.obj",                      // Luna
    ]
}
