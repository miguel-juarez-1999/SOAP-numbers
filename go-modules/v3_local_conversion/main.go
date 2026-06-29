package main

import (
	"fmt"
	"net/http"
	"strconv"
	"strings"
)

func numeroALetras(num int) string {
	if num == 0 {
		return "cero"
	}

	unidades := []string{"", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve"}
	especiales := []string{"diez", "once", "doce", "trece", "catorce", "quince", "dieciséis", "diecisiete", "dieciocho", "diecinueve"}
	decenas := []string{"", "", "veinte", "treinta", "cuarenta", "cincuenta", "sesenta", "setenta", "ochenta", "noventa"}
	cientos := []string{"", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos", "seiscientos", "setecientos", "ochocientos", "novecientos"}

	var convertir func(int) string
	convertir = func(n int) string {
		if n < 10 {
			return unidades[n]
		} else if n < 20 {
			return especiales[n-10]
		} else if n < 30 {
			if n == 20 {
				return "veinte"
			}
			return "veintí" + unidades[n-20]
		} else if n < 100 {
			u := n % 10
			if u > 0 {
				return decenas[n/10] + " y " + unidades[u]
			}
			return decenas[n/10]
		} else if n < 1000 {
			if n == 100 {
				return "cien"
			}
			u := n % 100
			if u > 0 {
				return cientos[n/100] + " " + convertir(u)
			}
			return cientos[n/100]
		} else if n < 1000000 {
			miles := n / 1000
			u := n % 1000
			resMiles := ""
			if miles == 1 {
				resMiles = "mil"
			} else {
				resMiles = convertir(miles) + " mil"
			}
			if u > 0 {
				return resMiles + " " + convertir(u)
			}
			return resMiles
		}
		return "Número fuera de rango"
	}

	return strings.TrimSpace(convertir(num))
}

func conintlHandler(w http.ResponseWriter, r *http.Request) {
	numeroStr := r.URL.Query().Get("n")
	if numeroStr == "" {
		http.Error(w, "Por favor, proporciona un número usando '?n=valor' en la URL.", http.StatusBadRequest)
		return
	}

	numero, err := strconv.Atoi(numeroStr)
	if err != nil || numero < 0 {
		http.Error(w, "Por favor, proporciona un número entero positivo válido.", http.StatusBadRequest)
		return
	}

	resultado := numeroALetras(numero)
	w.Header().Set("Content-Type", "text/plain; charset=utf-8")
	fmt.Fprint(w, resultado)
}

func main() {
	http.HandleFunc("/conintl", conintlHandler)
	fmt.Println("Servidor Go V3 corriendo en http://localhost:8000")
	http.ListenAndServe(":8000", nil)
}