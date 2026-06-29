use Mojolicious::Lite;
use Mojo::UserAgent;
use Mojo::Util 'url_escape';

get '/clisoap2' => sub {
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
            my $texto_ingles = $dom->at('NumberToWordsResult')->text;
            $texto_ingles =~ s/^\s+|\s+$//g; 


            my $texto_escapado = url_escape($texto_ingles);
            my $translate_url = "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q=$texto_escapado";
            
            my $tx_trans = $ua->get($translate_url);
            if (my $res_trans = $tx_trans->result) {
                if ($res_trans->is_success) {
                    my $json = $res_trans->json;
                    my $texto_espanol = $json->[0]->[0]->[0];
                    return $c->render(text => $texto_espanol);
                }
            }
            
      
            return $c->render(text => $texto_ingles);
        }
    }

    return $c->render(text => "Error al conectar con el servicio SOAP.", status => 500);
};

app->start('daemon', '-l', 'http://localhost:8000');