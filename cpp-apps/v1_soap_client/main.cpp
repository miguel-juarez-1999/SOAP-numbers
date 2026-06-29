#define _WINSOCK_DEPRECATED_NO_WARNINGS
#include <iostream>
#include <string>
#include <vector>
#include <winsock2.h>
#include <wininet.h>

#pragma comment(lib, "ws2_32.lib")
#pragma comment(lib, "wininet.lib")

std::string ejecutar_peticion_https(const std::string& numero) {
    HINTERNET hSession = InternetOpenA("HttpClient", INTERNET_OPEN_TYPE_DIRECT, NULL, NULL, 0);
    if (!hSession) return "";

    HINTERNET hConnect = InternetConnectA(hSession, "www.dataaccess.com", INTERNET_DEFAULT_HTTPS_PORT, NULL, NULL, INTERNET_SERVICE_HTTP, 0, 0);
    if (!hConnect) {
        InternetCloseHandle(hSession);
        return "";
    }

    HINTERNET hRequest = HttpOpenRequestA(hConnect, "POST", "/webservicesserver/NumberConversion.wso", NULL, NULL, NULL, INTERNET_FLAG_SECURE, 0);
    if (!hRequest) {
        InternetCloseHandle(hConnect);
        InternetCloseHandle(hSession);
        return "";
    }

    std::string soap_body = "<?xml version=\"1.0\" encoding=\"utf-8\"?>"
                            "<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\">"
                            "  <soap:Body>"
                            "    <NumberToWords xmlns=\"http://www.dataaccess.com/webservicesserver/\">"
                            "      <ubiNum>" + numero + "</ubiNum>"
                            "    </NumberToWords>"
                            "  </soap:Body>"
                            "</soap:Envelope>";

    std::string headers = "Content-Type: text/xml; charset=utf-8\r\n";

    BOOL bSend = HttpSendRequestA(hRequest, headers.c_str(), (DWORD)headers.length(), (LPVOID)soap_body.c_str(), (DWORD)soap_body.length());
    if (!bSend) {
        InternetCloseHandle(hRequest);
        InternetCloseHandle(hConnect);
        InternetCloseHandle(hSession);
        return "";
    }

    std::string respuesta;
    char buffer[4096];
    DWORD bytesRead = 0;
    while (InternetReadFile(hRequest, buffer, sizeof(buffer) - 1, &bytesRead) && bytesRead > 0) {
        buffer[bytesRead] = '\0';
        respuesta.append(buffer, bytesRead);
    }

    InternetCloseHandle(hRequest);
    InternetCloseHandle(hConnect);
    InternetCloseHandle(hSession);
    return respuesta;
}

std::string extraer_valor(const std::string& xml, const std::string& tag) {
    std::string start_tag = "<" + tag + ">";
    std::string end_tag = "</" + tag + ">";
    
    size_t start_pos = xml.find(start_tag);
    if (start_pos == std::string::npos) {
        size_t m_start_pos = xml.find("<m:" + tag + ">");
        if (m_start_pos != std::string::npos) {
            start_tag = "<m:" + tag + ">";
            end_tag = "</m:" + tag + ">";
            start_pos = m_start_pos;
        } else {
            return "";
        }
    }

    start_pos += start_tag.length();
    size_t end_pos = xml.find(end_tag, start_pos);
    if (end_pos == std::string::npos) return "";

    return xml.substr(start_pos, end_pos - start_pos);
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

    std::cout << "Servidor C++ V1 corriendo en http://localhost:8000" << std::endl;

    while (true) {
        SOCKET client_fd = accept(server_fd, nullptr, nullptr);
        if (client_fd == INVALID_SOCKET) continue;

        char recv_buf[2048] = {0};
        recv(client_fd, recv_buf, sizeof(recv_buf) - 1, 0);
        std::string request_str(recv_buf);

        size_t pos_n = request_str.find("?n=");
        if (pos_n == std::string::npos || request_str.find("GET /clisoap1") == std::string::npos) {
            std::string body = "Por favor, proporciona un numero usando '?n=valor' en la URL.";
            std::string res = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nContent-Length: " + std::to_string(body.length()) + "\r\n\r\n" + body;
            send(client_fd, res.c_str(), (int)res.length(), 0);
            closesocket(client_fd);
            continue;
        }

        pos_n += 3;
        size_t pos_space = request_str.find(" ", pos_n);
        std::string numero = request_str.substr(pos_n, pos_space - pos_n);

        std::string soap_response = ejecutar_peticion_https(numero);
        std::string resultado = extraer_valor(soap_response, "NumberToWordsResult");

        if (resultado.empty()) {
            resultado = "Error al procesar la solicitud SOAP en C++.";
        }

        std::string http_response = "HTTP/1.1 200 OK\r\n"
                                    "Content-Type: text/plain; charset=utf-8\r\n"
                                    "Content-Length: " + std::to_string(resultado.length()) + "\r\n"
                                    "Connection: close\r\n\r\n" + resultado;

        send(client_fd, http_response.c_str(), (int)http_response.length(), 0);
        closesocket(client_fd);
    }

    closesocket(server_fd);
    WSACleanup();
    return 0;
}