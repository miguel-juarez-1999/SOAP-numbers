require 'sinatra'
require 'net/http'
require 'uri'
require 'rexml/document'
require 'openssl'

set :port, 8000

get '/clisoap1' do
  numero = params['n']
  
  if numero.nil? || numero.empty?
    status 400
    return "Por favor, proporciona un número usando '?n=valor' en la URL."
  end

  wsdl_url = URI.parse("https://www.dataaccess.com/webservicesserver/NumberConversion.wso")
  
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

  http = Net::HTTP.new(wsdl_url.host, wsdl_url.port)
  http.use_ssl = true
  http.verify_mode = OpenSSL::SSL::VERIFY_NONE
  http.open_timeout = 10
  http.read_timeout = 10
  
  request = Net::HTTP::Post.new(wsdl_url.path)
  request.body = soap_envelope
  request.content_type = 'text/xml; charset=utf-8'
  
  begin
    response = http.request(request)
    
    if response.code != "200"
      status 500
      return "Error del servidor SOAP: Código #{response.code}"
    end

    doc = REXML::Document.new(response.body)
    resultado = REXML::XPath.first(doc, "//*[local-name()='NumberToWordsResult']").text
    resultado.strip
  rescue StandardError => e
    status 500
    "Error de conexión en Ruby: #{e.message}"
  end
end