package main

import (
	"bytes"
	"encoding/json"
	"encoding/xml"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strings"
)

type SoapEnvelope struct {
	XMLName xml.Name `xml:"Envelope"`
	Body    SoapBody `xml:"Body"`
}

type SoapBody struct {
	Response NumberToWordsResponse `xml:"NumberToWordsResponse"`
}

type NumberToWordsResponse struct {
	Result string `xml:"NumberToWordsResult"`
}

func clisoap2Handler(w http.ResponseWriter, r *http.Request) {
	numero := r.URL.Query().Get("n")
	if numero == "" {
		http.Error(w, "Por favor, proporciona un número usando '?n=valor' en la URL.", http.StatusBadRequest)
		return
	}

	wsdlURL := "https://www.dataaccess.com/webservicesserver/NumberConversion.wso"

	soapEnvelope := fmt.Sprintf(`<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
  <soap:Body>
    <NumberToWords xmlns="http://www.dataaccess.com/webservicesserver/">
      <ubiNum>%s</ubiNum>
    </NumberToWords>
  </soap:Body>
</soap:Envelope>`, numero)

	req, err := http.NewRequest("POST", wsdlURL, bytes.NewBufferString(soapEnvelope))
	if err != nil {
		http.Error(w, "Error al crear la petición: "+err.Error(), http.StatusInternalServerError)
		return
	}

	req.Header.Set("Content-Type", "text/xml; charset=utf-8")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		http.Error(w, "Error al conectar con el servicio SOAP: "+err.Error(), http.StatusInternalServerError)
		return
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		http.Error(w, "Error al leer la respuesta: "+err.Error(), http.StatusInternalServerError)
		return
	}

	var envelope SoapEnvelope
	err = xml.Unmarshal(body, &envelope)
	if err != nil {
		http.Error(w, "Error al parsear el XML: "+err.Error(), http.StatusInternalServerError)
		return
	}

	textoIngles := strings.TrimSpace(envelope.Body.Response.Result)

	translateURL := fmt.Sprintf("https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q=%s", url.QueryEscape(textoIngles))
	
	respTrans, err := client.Get(translateURL)
	if err != nil {
		w.Header().Set("Content-Type", "text/plain; charset=utf-8")
		fmt.Fprint(w, textoIngles)
		return
	}
	defer respTrans.Body.Close()

	bodyTrans, err := io.ReadAll(respTrans.Body)
	if err != nil {
		w.Header().Set("Content-Type", "text/plain; charset=utf-8")
		fmt.Fprint(w, textoIngles)
		return
	}

	var data []interface{}
	err = json.Unmarshal(bodyTrans, &data)
	if err != nil || len(data) == 0 {
		w.Header().Set("Content-Type", "text/plain; charset=utf-8")
		fmt.Fprint(w, textoIngles)
		return
	}

	firstArray, ok := data[0].([]interface{})
	if ok && len(firstArray) > 0 {
		secondArray, ok := firstArray[0].([]interface{})
		if ok && len(secondArray) > 0 {
			textoEspanol, ok := secondArray[0].(string)
			if ok {
				w.Header().Set("Content-Type", "text/plain; charset=utf-8")
				fmt.Fprint(w, strings.TrimSpace(textoEspanol))
				return
			}
		}
	}

	w.Header().Set("Content-Type", "text/plain; charset=utf-8")
	fmt.Fprint(w, textoIngles)
}

func main() {
	http.HandleFunc("/clisoap2", clisoap2Handler)
	fmt.Println("Servidor Go V2 corriendo en http://localhost:8000")
	http.ListenAndServe(":8000", nil)
}