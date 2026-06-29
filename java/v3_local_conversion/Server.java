import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpServer;
import java.io.IOException;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.net.URI;
import java.nio.charset.StandardCharsets;

public class Server {
    public static void main(String[] args) throws IOException {
        HttpServer server = HttpServer.create(new InetSocketAddress(8000), 0);
        server.createContext("/conintl", new ConIntlHandler());
        server.setExecutor(null);
        System.out.println("Servidor Java V3 corriendo en http://localhost:8000");
        server.start();
    }

    static class ConIntlHandler implements HttpHandler {
        @Override
        public void handle(HttpExchange exchange) throws IOException {
            URI requestURI = exchange.getRequestURI();
            String query = requestURI.getQuery();
            String numeroStr = "";

            if (query != null && query.contains("n=")) {
                String[] pairs = query.split("&");
                for (String pair : pairs) {
                    String[] idx = pair.split("=");
                    if (idx.length > 1 && idx[0].equals("n")) {
                        numeroStr = idx[1];
                        break;
                    }
                }
            }

            if (numeroStr.isEmpty()) {
                String errorMsg = "Por favor, proporciona un número usando '?n=valor' en la URL.";
                exchange.sendResponseHeaders(400, errorMsg.length());
                OutputStream os = exchange.getResponseBody();
                os.write(errorMsg.getBytes());
                os.close();
                return;
            }

            String resultado = "";
            try {
                int numero = Integer.parseInt(numeroStr);
                if (numero < 0) {
                    resultado = "Por favor, proporciona un número entero positivo válido.";
                    exchange.sendResponseHeaders(400, resultado.length());
                } else {
                    resultado = numeroALetras(numero);
                    exchange.getResponseHeaders().set("Content-Type", "text/plain; charset=utf-8");
                    exchange.sendResponseHeaders(200, resultado.getBytes(StandardCharsets.UTF_8).length);
                }
                OutputStream os = exchange.getResponseBody();
                os.write(resultado.getBytes(StandardCharsets.UTF_8));
                os.close();
            } catch (NumberFormatException e) {
                resultado = "Por favor, proporciona un número entero positivo válido.";
                exchange.sendResponseHeaders(400, resultado.length());
                OutputStream os = exchange.getResponseBody();
                os.write(resultado.getBytes());
                os.close();
            }
        }

        private String numeroALetras(int num) {
            if (num == 0) return "cero";

            String[] unidades = {"", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve"};
            String[] especiales = {"diez", "once", "doce", "trece", "catorce", "quince", "dieciséis", "diecisiete", "dieciocho", "diecinueve"};
            String[] decenas = {"", "", "veinte", "treinta", "cuarenta", "cincuenta", "sesenta", "setenta", "ochenta", "noventa"};
            String[] cientos = {"", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos", "seiscientos", "setecientos", "ochocientos", "novecientos"};

            return convertir(num, unidades, especiales, decenas, cientos).trim();
        }

        private String convertir(int n, String[] u, String[] esp, String[] d, String[] c) {
            if (n < 10) return u[n];
            if (n < 20) return esp[n - 10];
            if (n < 30) return (n == 20) ? "veinte" : "veintí" + u[n - 20];
            if (n < 100) {
                int resto = n % 10;
                return d[n / 10] + (resto > 0 ? " y " + u[resto] : "");
            }
            if (n < 1000) {
                if (n == 100) return "cien";
                int resto = n % 100;
                return c[n / 100] + (resto > 0 ? " " + convertir(resto, u, esp, d, c) : "");
            }
            if (n < 1000000) {
                int miles = n / 1000;
                int resto = n % 1000;
                String resMiles = (miles == 1) ? "mil" : convertir(miles, u, esp, d, c) + " mil";
                return resMiles + (resto > 0 ? " " + convertir(resto, u, esp, d, c) : "");
            }
            return "Número fuera de rango";
        }
    }
}