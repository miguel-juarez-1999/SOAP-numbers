use Mojolicious::Lite;
use Mojo::UserAgent;

get '/clisoap1' => sub {
    my $c = shift;
    my $numero = $c->param('n');

    if (!$numero) {
        return $c->render(text => "Por favor, proporciona un número usando '?n=valor' en la URL.", status => 400);
    }

    my $wsdl_url = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso';

    my $soap_envelope = <<"XML";
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
  <soap:Body>
    <NumberToWords xmlns="http://www.dataaccess.com/webservicesserver/">
      <ubiNum>$numero</ubiNum>
    </NumberToWords>
  </soap:Body>
</soap:Envelope>
XML

    my $ua = Mojo::UserAgent->new;
    my $tx = $ua->post($wsdl_url => {'Content-Type' => 'text/xml; charset=utf-8'} => $soap_envelope);

    if (my $res = $tx->result) {
        if ($res->is_success) {
            my $dom = $res->dom;
            my $resultado = $dom->at('NumberToWordsResult')->text;
            return $c->render(text => $resultado);
        }
    }

    return $c->render(text => "Error al conectar con el servicio SOAP.", status => 500);
};

app->start('daemon', '-l', 'http://localhost:8000');