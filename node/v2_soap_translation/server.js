const express = require('express');
const soap = require('soap');
const { translate } = require('google-translate-api-x');
const app = express();
const port = 8000;

const wsdlUrl = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL";

app.get('/clisoap2', (req, res) => {
    const numero = req.query.n;

    if (!numero) {
        return res.status(400).send("Por favor, proporciona un número usando '?n=valor' en la URL.");
    }

    soap.createClient(wsdlUrl, function(err, client) {
        if (err) {
            return res.status(500).send(err.message);
        }

        client.NumberToWords({ ubiNum: numero }, function(err, result) {
            if (err) {
                return res.status(500).send(err.message);
            }

            const textoIngles = result.NumberToWordsResult;

            translate(textoIngles, { from: 'en', to: 'es' })
                .then(traduccion => {
                    res.send(traduccion.text);
                })
                .catch(error => {
                    res.status(500).send(error.message);
                });
        });
    });
});

app.listen(port, () => {
    console.log(`Servidor Node V2 corriendo en http://localhost:${port}`);
});