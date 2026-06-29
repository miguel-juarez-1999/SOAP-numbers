use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn convertir(n: i32) -> String {
    let unidades = ["", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve"];
    let especiales = ["diez", "once", "doce", "trece", "catorce", "quince", "dieciseis", "diecisiete", "dieciocho", "diecinueve"];
    let decenas = ["", "", "veinte", "treinta", "cuarenta", "cincuenta", "sesenta", "setenta", "ochenta", "noventa"];
    let cientos = ["", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos", "seiscientos", "setecientos", "ochocientos", "novecientos"];

    if n < 10 { return unidades[n as usize].to_string(); }
    if n < 20 { return especiales[(n - 10) as usize].to_string(); }
    if n < 30 { 
        return if n == 20 { "veinte".to_string() } else { format!("veinti{}", unidades[(n - 20) as usize]) };
    }
    if n < 100 {
        let resto = n % 10;
        return format!("{}{}", decenas[(n / 10) as usize], if resto > 0 { format!(" y {}", unidades[resto as usize]) } else { "".to_string() });
    }
    if n < 1000 {
        if n == 100 { return "cien".to_string(); }
        let resto = n % 100;
        return format!("{}{}", cientos[(n / 100) as usize], if resto > 0 { format!(" {}", convertir(resto)) } else { "".to_string() });
    }
    if n < 1000000 {
        let miles = n / 1000;
        let resto = n % 1000;
        let res_miles = if miles == 1 { "mil".to_string() } else { format!("{} mil", convertir(miles)) };
        return format!("{}{}", res_miles, if resto > 0 { format!(" {}", convertir(resto)) } else { "".to_string() });
    }
    "Numero fuera de rango".to_string()
}

fn numero_a_letras(num: i32) -> String {
    if num == 0 { return "cero".to_string(); }
    convertir(num)
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    if stream.read(&mut buffer).is_err() { return; }
    let request_str = String::from_utf8_lossy(&buffer);

    if !request_str.contains("GET /conintl") || !request_str.contains("?n=") {
        let body = "Por favor, proporciona un numero usando '?n=valor' en la URL.";
        let response = format!(
            "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(), body
        );
        let _ = stream.write_all(response.as_bytes());
        return;
    }

    let pos_n = request_str.find("?n=").unwrap() + 3;
    let pos_space = request_str[pos_n..].find(' ').unwrap() + pos_n;
    let numero_str = &request_str[pos_n..pos_space];

    let resultado_final = match numero_str.parse::<i32>() {
        Ok(numero) => {
            if numero < 0 {
                "Por favor, proporciona un numero entero positivo valido.".to_string()
            } else {
                numero_a_letras(numero)
            }
        },
        Err(_) => "Por favor, proporciona un numero entero positivo valido.".to_string(),
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        resultado_final.len(), resultado_final
    );

    let _ = stream.write_all(response.as_bytes());
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Servidor Rust V3 corriendo en http://localhost:8000");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream);
        }
    }
}