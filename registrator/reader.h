#ifndef RFID_READER_H
#define RFID_READER_H

#include <MFRC522v2.h>
#include <MFRC522DriverSPI.h>
#include <MFRC522DriverPinSimple.h>
#include <cstddef>

#include "card.h"
#include "spi_bus.h"



#ifndef RFID_CS
#define RFID_CS  5
#endif  // RFID_CS



struct block {
  std::byte address;
  std::byte size;
  std::byte data[18];
};

void get_uid(std::byte arr[], size_t size, String& str) {
  for (size_t i = 0; i < size; i++) {
    str.concat(String(arr[i] < 0x10 ? "0" : " "));
    str.concat(String(arr[i], HEX));
  }
}


class RFIDReader {
private:
  std::size_t cs;
  MFRC522DriverPinSimple ss_pin;
  MFRC522DriverSPI driver;  // Create SPI driver
  MFRC522 mfrc522;          // Create MFRC522 instance
  MFRC522::MIFARE_Key key;

public:
  RFIDReader(std::size_t cs = RFID_CS) {
    cs = cs;
    ss_pin = MFRC522DriverPinSimple(cs);
    driver = MFRC522DriverSPI(ss_pin);
    
  }

  void setup() {
    add_peripheral(cs);
    enable_peripheral(cs);
    for (std::size_t i = 0; i < 6; i++) {
      key.keyByte[i] = 0xFF;
    }
    mfrc522.PCD_Init();    // Init MFRC522 board.
    disable_peripheral(cs);
  }


  void read_block(struct block& blk) {
    if (mfrc522.PCD_Authenticate(0x60, blk.address, &key, &(mfrc522.uid)) != 0) {
      Serial.println("Auth failed");
      return;
    }

    if(mfrc522.MIFARE_Read(blk.address, blk.data, &(blk.size)) != 0) {
      Serial.println("Read err");
      return;
    }

    Serial.println((char)blk.data[0]);
  }

  void poll() {
    enable_peripheral(cs);
      // Reset the loop if no new card present on the sensor/reader. This saves the entire process when idle.
    if (!mfrc522.PICC_IsNewCardPresent()) {
    
      return;
    }
  
    // Select one of the cards.
    if (!mfrc522.PICC_ReadCardSerial()) {
      return;
    }
  
    String uid = "";
    get_uid(mfrc522.uid.uidByte, mfrc522.uid.size, uid);
    Serial.println(uid);
    struct block blk {
      .address = 2,
      .size = 18,
      .data = {}
    };
  
    read_block(blk);
  
    for (byte i = 0; i < 18; i++) {
        Serial.print((char)blk.data[i]);  // Print as character
    }
    Serial.println();
  
    mfrc522.PICC_HaltA();
    mfrc522.PCD_StopCrypto1();
    disable_peripheral(RFID_CS);
  }


};


#endif  // RFID_READER_H