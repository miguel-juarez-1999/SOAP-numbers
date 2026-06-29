var builder = WebApplication.CreateBuilder(args);
var app = builder.Build();

string NumeroALetras(long num)
{
    if (num == 0) return "cero";

    string[] unidades = { "", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve" };
    string[] decenas = { "", "diez", "veinte", "treinta", "cuarenta", "cincuenta", "sesenta", "setenta", "ochenta", "noventa" };
    string[] especiales = { "diez", "once", "doce", "trece", "catorce", "quince", "dieciséis", "diecisiete", "diechocho", "diecinueve" };
    string[] cientos = { "", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos", "seiscientos", "setecientos", "ochocientos", "novecientos" };

    string Convertir(long n)
    {
        if (n < 10) return unidades[n];
        if (n >= 10 && n < 20) return especiales[n - 10];
        if (n >= 20 && n < 30) return n == 20 ? "veinte" : "veintí" + unidades[n - 20];
        if (n >= 30 && n < 100)
        {
            long u = n % 10;
            return decenas[n / 10] + (u > 0 ? " y " + unidades[u] : "");
        }
        if (n >= 100 && n < 1000)
        {
            if (n == 100) return "cien";
            long u = n % 100;
            return cientos[n / 100] + (u > 0 ? " " + Convertir(u) : "");
        }
        if (n >= 1000 && n < 1000000)
        {
            long miles = n / 1000;
            long u = n % 1000;
            string resMiles = miles == 1 ? "mil" : Convertir(miles) + " mil";
            return resMiles + (u > 0 ? " " + Convertir(u) : "");
        }
        return "Número fuera de rango";
    }

    return Convertir(num).Trim();
}

app.MapGet("/conintl", (string n) =>
{
    if (!long.TryParse(n, out long numero) || numero < 0)
    {
        return Results.BadRequest("Por favor, proporciona un número entero positivo válido usando '?n=valor' en la URL.");
    }

    return Results.Ok(NumeroALetras(numero));
});

app.Run("http://localhost:8000");