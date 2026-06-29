use Mojolicious::Lite;

sub numero_a_letras {
    my ($num) = @_;
    return "cero" if $num == 0;

    my @unidades  = ("", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve");
    my @especiales = ("diez", "once", "doce", "trece", "catorce", "quince", "dieciséis", "diecisiete", "dieciocho", "diecinueve");
    my @decenas    = ("", "", "veinte", "treinta", "cuarenta", "cincuenta", "sesenta", "setenta", "ochenta", "noventa");
    my @cientos    = ("", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos", "seiscientos", "setecientos", "ochocientos", "novecientos");

    my $convertir;
    $convertir = sub {
        my ($n) = @_;

        if ($n < 10) {
            return $unidades[$n];
        }
        elsif ($n < 20) {
            return $especiales[$n - 10];
        }
        elsif ($n < 30) {
            return $n == 20 ? "veinte" : "veintí" . $unidades[$n - 20];
        }
        elsif ($n < 100) {
            my $u = $n % 10;
            return $decenas[int($n / 10)] . ($u > 0 ? " y " . $unidades[$u] : "");
        }
        elsif ($n < 1000) {
            if ($n == 100) { return "cien"; }
            my $u = $n % 100;
            return $cientos[int($n / 100)] . ($u > 0 ? " " . $convertir->($u) : "");
        }
        elsif ($n < 1000000) {
            my $miles = int($n / 1000);
            my $u = $n % 1000;
            my $res_miles = $miles == 1 ? "mil" : $convertir->($miles) . " mil";
            return $res_miles . ($u > 0 ? " " . $convertir->($u) : "");
        }
        else {
            return "Número fuera de rango";
        }
    };

    my $resultado = $convertir->($num);
    $resultado =~ s/^\s+|\s+$//g; # Limpiar espacios
    return $resultado;
}

get '/conintl' => sub {
    my $c = shift;
    my $numero_str = $c->param('n');

    if (!defined $numero_str || $numero_str eq '' || $numero_str < 0) {
        return $c->render(text => "Por favor, proporciona un número entero positivo válido usando '?n=valor' en la URL.", status => 400);
    }

    my $resultado = numero_a_letras(int($numero_str));
    return $c->render(text => $resultado);
};

app->start('daemon', '-l', 'http://localhost:8000');