use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct  Libro {
    titulo: String,
    total_paginas: u32,
    autores: Vec<String>,
    generos: Vec<String>,
    precios: Vec<Precio>
}

#[derive(Serialize, Deserialize, Debug)]
struct  Precio {
    precio: f32,
    tipo: String,
    moneda: String,
}


fn main() {
    let libro = Libro {
        titulo: String::from("The Pragmatic Programmer"),
        total_paginas: 320,
        autores: vec![
            String::from("David Thomas"),
            String::from("Andrew Hunt")
        ],
        generos: vec![
            String::from("Programming"),
            String::from("IT")
        ],
        precios: vec![
            Precio {
                precio: 35.00,
                tipo: String::from("Tapa Dura"),
                moneda: String::from("USD")
            },
            Precio {
                precio: 15.00,
                tipo: String::from("digital"),
                moneda: String::from("USD")
            }
        ]
    };

    // Importante, en este caso como tenemos el contro, podemos
    // utilizar directamente el unwrap, en caso contrario debemos controlar
    // el error, pero por sencillez no lo haremos en este ejemplo.
    let libro_convertido_a_json = serde_json::to_string(&libro).unwrap();
    println!("{}", libro_convertido_a_json);
}
