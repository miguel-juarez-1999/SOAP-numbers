require 'sinatra'

set :port, 8000

def numero_a_letras(num)
  return "cero" if num == 0

  unidades = ["", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve"]
  especiales = ["diez", "once", "doce", "trece", "catorce", "quince", "dieciséis", "diecisiete", "dieciocho", "diecinueve"]
  decenas = ["", "", "veinte", "treinta", "cuarenta", "cincuenta", "sesenta", "setenta", "ochenta", "noventa"]
  cientos = ["", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos", "seiscientos", "setecientos", "ochocientos", "novecientos"]

  convertir = lambda do |n|
    if n < 10
      unidades[n]
    elsif n < 20
      especiales[n - 10]
    elsif n < 30
      n == 20 ? "veinte" : "veintí" + unidades[n - 20]
    elsif n < 100
      u = n % 10
      decenas[n / 10] + (u > 0 ? " y " + unidades[u] : "")
    elsif n < 1000
      if n == 100
        "cien"
      else
        u = n % 100
        cientos[n / 100] + (u > 0 ? " " + convertir.call(u) : "")
      end
    elsif n < 1000000
      miles = n / 1000
      u = n % 1000
      res_miles = miles == 1 ? "mil" : convertir.call(miles) + " mil"
      res_miles + (u > 0 ? " " + convertir.call(u) : "")
    else
      "Número fuera de rango"
    end
  end

  convertir.call(num).strip
end

get '/conintl' do
  numero_str = params['n']
  
  if numero_str.nil? || numero_str.empty? || numero_str.to_i < 0
    status 400
    return "Por favor, proporciona un número entero positivo válido usando '?n=valor' en la URL."
  end

  numero = numero_str.to_i
  numero_a_letras(numero)
end