#define _WINSOCK_DEPRECATED_NO_WARNINGS
#include <iostream>
#include <string>
#include <vector>
#include <winsock2.h>

#pragma comment(lib, "ws2_32.lib")

std::string unidades[] = {"", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve"};
std::string especiales[] = {"diez", "once", "doce", "trece", "catorce", "quince", "dieciseis", "diecisiete", "dieciocho", "diecinueve"};
std::string decenas[] = {"", "", "veinte", "treinta", "cuarenta", "cincuenta", "sesenta", "setenta", "ochenta", "noventa"};
std::string cientos[] = {"", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos", "seiscientos", "setecientos", "ochocientos", "novecientos"};

std::string convertir(int n) {
    if (n < 10) return unidades[n];
    if (n < 20) return especiales[n - 10];
    if (n < 30) return (n == 20) ? "veinte" : "veinti" + unidades[n - 20];
    if (n < 100) {
        int resto = n % 10;
        return decenas[n / 10] + (resto > 0 ? " y " + unidades[resto] : "");
    }
    if (n < 1000) {
        if (n == 100) return "cien";
        int resto = n % 100;
        return cientos[n / 100] + (resto > 0 ? " " + convertir(resto) : "");
    }
    if (n < 1000000) {
        int miles = n / 1000;
        int resto = n % 1000;
        std::string resMiles = (miles == 1) ? "mil" : convertir(miles) + " mil";
        return resMiles + (resto > 0 ? " " + convertir(resto) : "");
    }
    return "Numero fuera de rango";
}

std::string numeroALetras(int num) {
    if (num == 0) return "cero";
    return convertir(num);
}

int main() {
    WSADATA wsaData;
    if (WSAStartup(MAKEWORD(2, 2), &wsaData) != 0) {
        return 1;
    }

    SOCKET server_fd = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in server_addr;
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(8000);

    bind(server_fd, (struct sockaddr*)&server_addr, sizeof(server_addr));
    listen(server_fd, 10);

    std::cout << "Servidor C++ V3 corriendo en http://localhost:8000" << std::endl;

    while (true) {
        SOCKET client_fd = accept(server_fd, nullptr, nullptr);
        if (client_fd == INVALID_SOCKET) continue;

        char recv_buf[2048] = {0};
        recv(client_fd, recv_buf, sizeof(recv_buf) - 1, 0);
        std::string request_str(recv_buf);

        size_t pos_n = request_str.find("?n=");
        if (pos_n == std::string::npos || request_str.find("GET /conintl") == std::string::npos) {
            std::string body = "Por favor, proporciona un numero usando '?n=valor' en la URL.";
            std::string res = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nContent-Length: " + std::to_string(body.length()) + "\r\n\r\n" + body;
            send(client_fd, res.c_str(), (int)res.length(), 0);
            closesocket(client_fd);
            continue;
        }

        pos_n += 3;
        size_t pos_space = request_str.find(" ", pos_n);
        std::string numeroStr = request_str.substr(pos_n, pos_space - pos_n);

        std::string resultadoFinal;
        try {
            int numero = std::stoi(numeroStr);
            if (numero < 0) {
                resultadoFinal = "Por favor, proporciona un numero entero positivo valido.";
            } else {
                resultadoFinal = numeroALetras(numero);
            }
        } catch (...) {
            resultadoFinal = "Por favor, proporciona un numero entero positivo valido.";
        }

        std::string http_response = "HTTP/1.1 200 OK\r\n"
                                    "Content-Type: text/plain; charset=utf-8\r\n"
                                    "Content-Length: " + std::to_string(resultadoFinal.length()) + "\r\n"
                                    "Connection: close\r\n\r\n" + resultadoFinal;

        send(client_fd, http_response.c_str(), (int)http_response.length(), 0);
        closesocket(client_fd);
    }

    closesocket(server_fd);
    WSACleanup();
    return 0;
}