#include <SPI.h>
#include <MFRC522v2.h>
#include <MFRC522DriverSPI.h>
#include <MFRC522DriverPinSimple.h>

// Only SS pin, no RST connected
#define SS_PIN D8   // GPIO15

MFRC522DriverPinSimple ss_pin(SS_PIN);
MFRC522DriverSPI driver(ss_pin);
MFRC522 rfid(driver);

void setup() {
  Serial.begin(115200);  // high baud for faster debug
  delay(1000);

  Serial.println("==== ESP8266 + RC522 DEBUG ====");
  Serial.print("SPI Init... ");
  SPI.begin();
  Serial.println("DONE");

  Serial.print("RFID Init... ");
  rfid.PCD_Init();
  Serial.println("DONE");

  Serial.println("Place a card near the reader...");
}

void loop() {
  // Check if card is present
  if (!rfid.PICC_IsNewCardPresent()) {
    Serial.println("No card detected");
    delay(2000);
    return;
  }

  Serial.println("Card detected!");

  // Try reading UID
  if (!rfid.PICC_ReadCardSerial()) {
    Serial.println("Failed to read UID");
    delay(2000);
    return;
  }

  Serial.print("UID: ");
  for (byte i = 0; i < rfid.uid.size; i++) {
    Serial.print(rfid.uid.uidByte[i], HEX);
    Serial.print(" ");
  }
  Serial.println();

  // Halt PICC
  rfid.PICC_HaltA();
  rfid.PCD_StopCrypto1();
  delay(2000);
}

