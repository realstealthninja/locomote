#include <MFRC522DriverPin.h>
#include <MFRC522DriverPinSimple.h>
#include <MFRC522DriverSPI.h>
#include <MFRC522v2.h>
#include <MFRC522Debug.h>

MFRC522DriverPinSimple ss_pin(5);
MFRC522DriverSPI driver{ ss_pin };


class Reader {
  MFRC522 mfrc522{ driver };

  bool has_card = false;
public:
  Reader();
  void loop() {
    if (!mfrc522.PICC_IsNewCardPresent()) {
      return;
    }

    if (!mfrc522.PICC_ReadCardSerial()) {
      has_card = false;
      return;
    }

    has_card = true;
  }

  void setup() {
    mfrc522.PCD_Init();
    MFRC522Debug::PCD_DumpVersionToSerial(mfrc522, Serial);
    Serial.println("Driver standby...");
  }

  void set_key(String key) {
  }

  void write_secret(String secret, String key) {
    if (!has_card) return;
    // get key
    MFRC522::MIFARE_Key _key;
    byte buffer[18];

    key.getBytes(_key.keyByte, sizeof(_key.keyByte));
    secret.getBytes(buffer, sizeof(buffer));

    mfrc522.PCD_Authenticate(MFRC522::PICC_Command::PICC_CMD_MF_AUTH_KEY_A, 1, &_key, &mfrc522.uid);
    mfrc522.MIFARE_Write(1, buffer, 16);
    mfrc522.PCD_StopCrypto1();
  }

  String read_secret(String key) {
    MFRC522::MIFARE_Key _key;
    key.getBytes(_key.keyByte, sizeof(_key.keyByte));

    mfrc522.PCD_Authenticate(MFRC522::PICC_Command::PICC_CMD_MF_AUTH_KEY_A, 1, &_key, &mfrc522.uid);

    byte buffer[18];
    byte size = 18;
    mfrc522.MIFARE_Read(1, buffer, &size);
    mfrc522.PCD_StopCrypto1();

    return String((char*)buffer);
  }

  String read_uid() {
    String uid;
    MFRC522Debug::PrintUID(Serial, mfrc522.uid);
    return uid;
  }

  void stop_card() {
    mfrc522.PICC_HaltA();
  }
}
