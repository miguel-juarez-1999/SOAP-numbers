const express = require('express');
const soap = require('soap');
const app = express();
const port = 8000;

// URL del WSDL público que vimos en clase
const wsdlUrl = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL";

// Ruta equivalente a tu clase: http://localhost:8000/clisoap1?n=10
app.get('/clisoap1', (req, res) => {
    const numero = req.query.n;

    // Si el usuario no manda el parámetro 'n', le avisamos
    if (!numero) {
        return res.status(400).send("Por favor, proporciona un número usando '?n=valor' en la URL.");
    }

    // Creamos el cliente SOAP con la librería
    soap.createClient(wsdlUrl, function(err, client) {
        if (err) {
            return res.status(500).send("Error al conectar con el servicio SOAP: " + err.message);
        }

        // Ejecutamos la función NumberToWords pasando 'ubiNum' (lo que pide el WSDL)
        client.NumberToWords({ ubiNum: numero }, function(err, result) {
            if (err) {
                return res.status(500).send("Error al procesar la solicitud SOAP: " + err.message);
            }

            // Respondemos con el resultado en inglés
            res.send(result.NumberToWordsResult);
        });
    });
});

app.listen(port, () => {
    console.log(`Servidor Node V1 corriendo en http://localhost:${port}`);
});