use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use reqwest::blocking::Client;

fn ejecutar_peticion_soap(numero: &str) -> Result<String, reqwest::Error> {
    let url = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso";
    
    let soap_body = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
  <soap:Body>
    <NumberToWords xmlns="http://www.dataaccess.com/webservicesserver/">
      <ubiNum>{}</ubiNum>
    </NumberToWords>
  </soap:Body>
</soap:Envelope>"#,
        numero
    );

    let client = Client::new();
    let res = client.post(url)
        .header("Content-Type", "text/xml; charset=utf-8")
        .body(soap_body)
        .send()?;

    Ok(res.text()?)
}

fn extraer_valor(xml: &str, tag: &str) -> String {
    let start_tag = format!("<{}>", tag);
    let end_tag = format!("</{}>", tag);
    let m_start_tag = format!("<m:{}>", tag);
    let m_end_tag = format!("</m:{}>", tag);

    if let Some(start_idx) = xml.find(&start_tag) {
        let start = start_idx + start_tag.len();
        if let Some(end) = xml[start..].find(&end_tag) {
            return xml[start..start + end].trim().to_string();
        }
    } else if let Some(start_idx) = xml.find(&m_start_tag) {
        let start = start_idx + m_start_tag.len();
        if let Some(end) = xml[start..].find(&m_end_tag) {
            return xml[start..start + end].trim().to_string();
        }
    }
    "".to_string()
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    if stream.read(&mut buffer).is_err() { return; }
    let request_str = String::from_utf8_lossy(&buffer);

    if !request_str.contains("GET /clisoap1") || !request_str.contains("?n=") {
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
    let numero = &request_str[pos_n..pos_space];

    let mut resultado = match ejecutar_peticion_soap(numero) {
        Ok(xml_res) => extraer_valor(&xml_res, "NumberToWordsResult"),
        Err(_) => "".to_string(),
    };

    if resultado.is_empty() {
        resultado = "Error al procesar la solicitud SOAP en Rust.".to_string();
    }

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        resultado.len(), resultado
    );

    let _ = stream.write_all(response.as_bytes());
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Servidor Rust V1 corriendo en http://localhost:8000");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream);
        }
    }
}