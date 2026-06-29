use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use reqwest::blocking::Client;

fn ejecutar_peticion_https(url: &str, metodo: &str, headers: Vec<(&str, &str)>, body: Option<String>) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let mut req = if metodo == "POST" {
        client.post(url)
    } else {
        client.get(url)
    };

    for (k, v) in headers {
        req = req.header(k, v);
    }

    if let Some(b) = body {
        req = req.body(b);
    }

    let res = req.send()?;
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

fn extraer_json_traduccion(json: &str) -> String {
    if let Some(start_idx) = json.find("[[[\"") {
        let start = start_idx + 4;
        if let Some(end_idx) = json[start..].find('"') {
            return json[start..start + end_idx].trim().to_string();
        }
    }
    "".to_string()
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    if stream.read(&mut buffer).is_err() { return; }
    let request_str = String::from_utf8_lossy(&buffer);

    if !request_str.contains("GET /clisoap2") || !request_str.contains("?n=") {
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

    let soap_url = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso";
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

    let soap_headers = vec![("Content-Type", "text/xml; charset=utf-8")];
    let soap_response = ejecutar_peticion_https(soap_url, "POST", soap_headers, Some(soap_body));

    let mut texto_ingles = match soap_response {
        Ok(xml_res) => extraer_valor(&xml_res, "NumberToWordsResult"),
        Err(_) => "".to_string(),
    };

    while texto_ingles.ends_with(' ') {
        texto_ingles.pop();
    }

    let mut resultado_final = texto_ingles.clone();

    if !texto_ingles.is_empty() {
        let texto_escapado = texto_ingles.replace(' ', "%20");
        let translate_url = format!(
            "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q={}",
            texto_escapado
        );

        if let Ok(trans_res) = ejecutar_peticion_https(&translate_url, "GET", vec![], None) {
            let texto_espanol = extraer_json_traduccion(&trans_res);
            if !texto_espanol.is_empty() {
                resultado_final = texto_espanol;
            }
        }
    } else {
        resultado_final = "Error al procesar la solicitud SOAP en Rust.".to_string();
    }

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        resultado_final.len(), resultado_final
    );

    let _ = stream.write_all(response.as_bytes());
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Servidor Rust V2 corriendo en http://localhost:8000");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream);
        }
    }
}