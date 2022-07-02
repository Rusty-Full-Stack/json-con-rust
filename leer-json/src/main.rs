use serde_json::{Result, Value};

fn main() -> Result<()> {
    let libro = r#"{
  "titulo": "The Pragmatic Programmer",
  "autores": [
    "David Thomas",
    "Andrew Hunt"
  ],
  "total_paginas": 352,
  "generos": [
    "programacion",
    "ingenieria",
    "educacion"
  ],
  "precios": [
    {
      "tipo": "digital",
      "precio": 15.00,
      "moneda": "USD"
    },
    {
      "tipo": "tapa dura",
      "precio": 35.50,
      "moneda": "USD"
    }
  ]
}"#;

    let libro_parsed: Value = serde_json::from_str(libro)?;
    const DESCUENTO: f64 = 10.00; // Valor de descuento, puedes cambiarlo a tu necesidad.

    println!("Libro procesado por Rust");
    println!("Titulo del libro: {}", libro_parsed["titulo"].as_str().unwrap());
    println!("PRECIOS:");
    for precio in libro_parsed["precios"].as_array().unwrap() {
        println!("=====================================================================");

        let precio_regular = precio["precio"].as_f64().unwrap();
        let precio_descuento = precio_regular - (precio_regular * (DESCUENTO / 100.00));

        println!("Tipo: {}", precio["tipo"].as_str().unwrap());
        println!("Precio Regular: {} {}", precio_regular, precio["moneda"]);
        println!("Precio 10% Descuento: {} {}", precio_descuento, precio["moneda"]);
    }

    Ok(())
}
