require 'sinatra'
require 'httparty'
require 'rexml/document'
require 'cgi'

set :port, 8000

get '/clisoap2' do
  numero = params['n']
  
  if numero.nil? || numero.empty?
    status 400
    return "Por favor, proporciona un número usando '?n=valor' en la URL."
  end

  wsdl_url = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso"
  
  soap_envelope = <<~XML
    <?xml version="1.0" encoding="utf-8"?>
    <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
      <soap:Body>
        <NumberToWords xmlns="http://www.dataaccess.com/webservicesserver/">
          <ubiNum>#{numero}</ubiNum>
        </NumberToWords>
      </soap:Body>
    </soap:Envelope>
  XML

  begin

    response = HTTParty.post(wsdl_url, 
      body: soap_envelope,
      headers: { 'Content-Type' => 'text/xml; charset=utf-8' },
      timeout: 10
    )

    if response.code != 200
      status 500
      return "Error del servidor SOAP: Código #{response.code}"
    end

    doc = REXML::Document.new(response.body)
    texto_ingles = REXML::XPath.first(doc, "//*[local-name()='NumberToWordsResult']").text.strip


    translate_url = "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q=#{CGI.escape(texto_ingles)}"
    translate_response = HTTParty.get(translate_url, timeout: 5)
    
    if translate_response.code == 200
      texto_espanol = translate_response.parsed_response[0][0][0]
      texto_espanol.strip
    else
      texto_ingles
    end

  rescue StandardError => e
    status 500
    "Error en la ejecución de Ruby V2: #{e.message}"
  end
end