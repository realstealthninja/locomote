#include <WiFi.h>
#include <HTTPClient.h>


const char* ssid = "Weaboo's S25 Ultra";
const char* password = "12345687";

const String HOST = "";

HTTPClient client;

void send_card() {
  http.begin(HOST + "/api/v1/cards/verify");
}