use std::path::Path;
use std::fs;

/// Validates that all texture files referenced in MTL files exist
pub fn validate_texture_paths() {
    println!("\n=== VALIDACIÓN DE TEXTURAS ===\n");
    
    let models_dir = "assets/models";
    
    // Define models and their expected textures
    let model_textures = vec![
        ("13913_Sun_v2_l3.obj", vec!["13913_Sun_diff.jpg"]),
        ("13902_Earth_v1_l3.obj", vec!["Earth_diff.jpg"]),
        ("13905_Jupiter_V1_l3.obj", vec!["Jupiter_diff.jpg"]),
        ("13907_Uranus_v2_l3.obj", vec!["13907_Uranus_planet_diff.JPG", "13907_Uranus_planet_diff.jpg"]),
        ("10464_Asteroid_v1_Iterations-2.obj", vec!["10464_Asteroid_v1_diffuse.jpg"]),
    ];
    
    for (model, textures) in model_textures {
        println!("📦 Modelo: {}", model);
        
        // Check if model exists
        let model_path = format!("{}/{}", models_dir, model);
        if Path::new(&model_path).exists() {
            println!("  ✓ Archivo OBJ encontrado");
        } else {
            println!("  ✗ Archivo OBJ NO encontrado");
            continue;
        }
        
        // Check textures
        let mut texture_found = false;
        for texture in textures {
            let texture_path = format!("{}/{}", models_dir, texture);
            if Path::new(&texture_path).exists() {
                println!("  ✓ Textura encontrada: {}", texture);
                texture_found = true;
                
                // Check file size
                if let Ok(metadata) = fs::metadata(&texture_path) {
                    let size = metadata.len();
                    println!("    Tamaño: {} KB", size / 1024);
                }
            } else {
                println!("  ✗ Textura NO encontrada: {}", texture);
            }
        }
        
        if !texture_found {
            println!("  ⚠ ADVERTENCIA: No se encontró ninguna textura válida para este modelo");
        }
        
        println!();
    }
    
    // List all files in the models directory
    println!("\n=== ARCHIVOS DISPONIBLES EN assets/models ===\n");
    if let Ok(entries) = fs::read_dir(models_dir) {
        let mut files: Vec<String> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        
        files.sort();
        
        for file in files {
            let extension = Path::new(&file).extension()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            
            match extension.to_lowercase().as_str() {
                "jpg" | "jpeg" | "png" => println!("🖼️  {}", file),
                "obj" => println!("📦 {}", file),
                "mtl" => println!("📄 {}", file),
                _ => println!("📋 {}", file),
            }
        }
    }
}

/// Validates texture loading at runtime
pub fn validate_texture_loading() {
    use crate::obj::Obj;
    
    println!("\n=== PRUEBA DE CARGA DE TEXTURAS ===\n");
    
    let models = vec![
        ("Sol", "assets/models/13913_Sun_v2_l3.obj"),
        ("Tierra", "assets/models/13902_Earth_v1_l3.obj"),
        ("Júpiter", "assets/models/13905_Jupiter_V1_l3.obj"),
        ("Urano", "assets/models/13907_Uranus_v2_l3.obj"),
        ("Asteroide", "assets/models/10464_Asteroid_v1_Iterations-2.obj"),
    ];
    
    for (name, path) in models {
        println!("🌍 Cargando: {}", name);
        match Obj::load(path) {
            Ok(obj) => {
                println!("  ✓ Modelo cargado correctamente");
                println!("  - Vértices: {}", obj.vertices.len());
                println!("  - Índices: {}", obj.indices.len());
                println!("  - Materiales: {}", obj.materials.len());
                
                // Check materials
                for (i, mat) in obj.materials.iter().enumerate() {
                    println!("    Material {}: {}", i, mat.name);
                    println!("      - Diffuse: ({:.2}, {:.2}, {:.2})", mat.diffuse.x, mat.diffuse.y, mat.diffuse.z);
                    if let Some(tex_path) = &mat.texture_path {
                        println!("      - Textura referenciada: {}", tex_path);
                    } else {
                        println!("      - Sin textura");
                    }
                }
                
                // Check if texture was loaded
                if let Some(texture) = &obj.texture {
                    println!("  ✓ Textura cargada exitosamente");
                    println!("    - Dimensiones: {}x{}", texture.width, texture.height);
                    println!("    - Datos: {} bytes", texture.data.len());
                } else {
                    println!("  ⚠ ADVERTENCIA: No se cargó textura para este modelo");
                }
            }
            Err(e) => {
                println!("  ✗ ERROR al cargar modelo: {:?}", e);
            }
        }
        println!();
    }
}
