

#include <stdint.h>
#include <ArduinoJson.h>
#include <WebSocketsClient.h>


const char* ssid = "Weaboo's S25 Ultra";
const char* password = "12345678:)";
const char* server = "ws://10.70.160.13/ws/registrar";

WebSocketsClient webSocket;

void webSocketEvent(WStype_t type, uint8_t* payload, size_t length) {
  switch (type) {
    case WStype_CONNECTED:
      Serial.println("Connected to WebSocket server");
      webSocket.sendTXT("Hello Server!");  // Send a message to the server
      break;
    case WStype_DISCONNECTED:
      Serial.println("Disconnected from WebSocket server");
      break;
    case WStype_TEXT:
      Serial.printf("Received text message: %s\n", payload);        
      break;
    case WStype_PONG:
      Serial.println("Received PONG");
      break;
    default:
      break;
  }
}

void web_setup() {

  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    delay(1000);
    Serial.println("Connecting to WiFi...");
  }
  Serial.println("Connected to WiFi!");
  Serial.print("IP Address: ");
  Serial.println(WiFi.localIP());

  webSocket.begin(server, 6542, "/ws");  // Connect to WebSocket server
  webSocket.onEvent(webSocketEvent);     // Register the event handler
}

void web_loop() {
  webSocket.loop();
}

void send(JsonDocument payload) {
  String strPayload;
  serializeJson(payload, strPayload);
  webSocket.sendTXT(strPayload);
}
