using System.Text;
using System.Xml.Linq;

var builder = WebApplication.CreateBuilder(args);
var app = builder.Build();

const string wsdlUrl = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso";

app.MapGet("/clisoap1", async (string n) =>
{
    if (string.IsNullOrEmpty(n))
    {
        return Results.BadRequest("Por favor, proporciona un número usando '?n=valor' en la URL.");
    }

    string soapEnvelope = $@"<?xml version=""1.0"" encoding=""utf-8""?>
    <soap:Envelope xmlns:soap=""http://schemas.xmlsoap.org/soap/envelope/"">
      <soap:Body>
        <NumberToWords xmlns=""http://www.dataaccess.com/webservicesserver/"">
          <ubiNum>{n}</ubiNum>
        </NumberToWords>
      </soap:Body>
    </soap:Envelope>";

    using var httpClient = new HttpClient();
    var content = new StringContent(soapEnvelope, Encoding.UTF8, "text/xml");
    
    var response = await httpClient.PostAsync(wsdlUrl, content);
    if (!response.IsSuccessStatusCode)
    {
        return Results.Problem("Error al conectar con el servicio SOAP.");
    }

    string xmlResult = await response.Content.ReadAsStringAsync();
    var doc = XDocument.Parse(xmlResult);
    XNamespace ns = "http://www.dataaccess.com/webservicesserver/";
    
    string resultText = doc.Descendants(ns + "NumberToWordsResult").FirstOrDefault()?.Value ?? "";

    return Results.Ok(resultText.Trim());
});

app.Run("http://localhost:8000");