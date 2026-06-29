const express = require('express');
const app = express();
const port = 8000;

function numeroALetras(num) {
    if (num === 0) return 'cero';

    const unidades = ['', 'uno', 'dos', 'tres', 'cuatro', 'cinco', 'seis', 'siete', 'ocho', 'nueve'];
    const decenas = ['', 'diez', 'veinte', 'treinta', 'cuarenta', 'cincuenta', 'sesenta', 'setenta', 'ochenta', 'noventa'];
    const especiales = ['diez', 'once', 'doce', 'trece', 'catorce', 'quince', 'dieciséis', 'diecisiete', 'dieciocho', 'diecinueve'];
    const cientos = ['', 'ciento', 'doscientos', 'trescientos', 'cuatrocientos', 'quinientos', 'seiscientos', 'setecientos', 'ochocientos', 'novecientos'];

    function convertir(n) {
        if (n < 10) return unidades[n];
        if (n >= 10 && n < 20) return especiales[n - 10];
        if (n >= 20 && n < 30) return n === 20 ? 'veinte' : 'veintí' + unidades[n - 20];
        if (n >= 30 && n < 100) {
            let u = n % 10;
            return decenas[Math.floor(n / 10)] + (u > 0 ? ' y ' + unidades[u] : '');
        }
        if (n >= 100 && n < 1000) {
            if (n === 100) return 'cien';
            let u = n % 100;
            return cientos[Math.floor(n / 100)] + (u > 0 ? ' ' + convertir(u) : '');
        }
        if (n >= 1000 && n < 1000000) {
            let miles = Math.floor(n / 1000);
            let u = n % 1000;
            let resMiles = miles === 1 ? 'mil' : convertir(miles) + ' mil';
            return resMiles + (u > 0 ? ' ' + convertir(u) : '');
        }
        return 'Número fuera de rango para el ejemplo';
    }

    return convertir(num).trim();
}

app.get('/conintl', (req, res) => {
    const numero = parseInt(req.query.n, 10);

    if (isNaN(numero) || numero < 0) {
        return res.status(400).send("Por favor, proporciona un número entero positivo válido usando '?n=valor' en la URL.");
    }

    res.send(numeroALetras(numero));
});

app.listen(port, () => {
    console.log(`Servidor Node V3 corriendo en http://localhost:${port}`);
});