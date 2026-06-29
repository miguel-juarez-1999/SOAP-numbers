const express = require('express');
const soap = require('soap');
const app = express();
const port = 8000;


const wsdlUrl = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL";


app.get('/clisoap1', (req, res) => {
    const numero = req.query.n;


    if (!numero) {
        return res.status(400).send("Por favor, proporciona un número usando '?n=valor' en la URL.");
    }

    soap.createClient(wsdlUrl, function(err, client) {
        if (err) {
            return res.status(500).send("Error al conectar con el servicio SOAP: " + err.message);
        }


        client.NumberToWords({ ubiNum: numero }, function(err, result) {
            if (err) {
                return res.status(500).send("Error al procesar la solicitud SOAP: " + err.message);
            }

            res.send(result.NumberToWordsResult);
        });
    });
});

app.listen(port, () => {
    console.log(`Servidor Node V1 corriendo en http://localhost:${port}`);
});