import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpServer;
import java.io.IOException;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Server {
    public static void main(String[] args) throws IOException {
        HttpServer server = HttpServer.create(new InetSocketAddress(8000), 0);
        server.createContext("/clisoap1", new CliSoap1Handler());
        server.setExecutor(null);
        System.out.println("Servidor Java V1 corriendo en http://localhost:8000");
        server.start();
    }

    static class CliSoap1Handler implements HttpHandler {
        @Override
        public void handle(HttpExchange exchange) throws IOException {
            URI requestURI = exchange.getRequestURI();
            String query = requestURI.getQuery();
            String numero = "";

            if (query != null && query.contains("n=")) {
                String[] pairs = query.split("&");
                for (String pair : pairs) {
                    String[] idx = pair.split("=");
                    if (idx.length > 1 && idx[0].equals("n")) {
                        numero = idx[1];
                        break;
                    }
                }
            }

            if (numero.isEmpty()) {
                String errorMsg = "Por favor, proporciona un número usando '?n=valor' en la URL.";
                exchange.sendResponseHeaders(400, errorMsg.length());
                OutputStream os = exchange.getResponseBody();
                os.write(errorMsg.getBytes());
                os.close();
                return;
            }

            String wsdlUrl = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso";
            String soapEnvelope = "<?xml version=\"1.0\" encoding=\"utf-8\"?>"
                    + "<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\">"
                    + "  <soap:Body>"
                    + "    <NumberToWords xmlns=\"http://www.dataaccess.com/webservicesserver/\">"
                    + "      <ubiNum>" + numero + "</ubiNum>"
                    + "    </NumberToWords>"
                    + "  </soap:Body>"
                    + "</soap:Envelope>";

            try {
                HttpClient client = HttpClient.newHttpClient();
                HttpRequest request = HttpRequest.newBuilder()
                        .uri(URI.create(wsdlUrl))
                        .header("Content-Type", "text/xml; charset=utf-8")
                        .POST(HttpRequest.BodyPublishers.ofString(soapEnvelope))
                        .build();

                HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());
                String body = response.body();

                Pattern pattern = Pattern.compile("<[^:]*:?NumberToWordsResult>([^<]+)</[^:]*:?NumberToWordsResult>", Pattern.CASE_INSENSITIVE);
                Matcher matcher = pattern.matcher(body);
                String resultado = "";

                if (matcher.find()) {
                    resultado = matcher.group(1).trim();
                } else {
                    resultado = "Error: No se pudo extraer el texto de la respuesta SOAP.";
                }

                exchange.getResponseHeaders().set("Content-Type", "text/plain; charset=utf-8");
                exchange.sendResponseHeaders(200, resultado.getBytes("UTF-8").length);
                OutputStream os = exchange.getResponseBody();
                os.write(resultado.getBytes("UTF-8"));
                os.close();

            } catch (Exception e) {
                String errorMsg = "Error en la solicitud: " + e.getMessage();
                exchange.sendResponseHeaders(500, errorMsg.length());
                OutputStream os = exchange.getResponseBody();
                os.write(errorMsg.getBytes());
                os.close();
            }
        }
    }
}