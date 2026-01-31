#ifndef RFID_READER_H
#define RFID_READER_H

#include <MFRC522v2.h>
#include <MFRC522DriverSPI.h>
#include <MFRC522DriverPinSimple.h>

#include "spi_bus.h"



#ifndef RFID_CS
#define RFID_CS  5
#define RFID_INT 13
#endif  // RFID_CS

MFRC522DriverPinSimple ss_pin(5);

MFRC522DriverSPI driver{ ss_pin };  // Create SPI driver
MFRC522 mfrc522{ driver };          // Create MFRC522 instance
MFRC522::MIFARE_Key key;

struct block {
  byte address;
  byte size;
  byte data[18];
};

void get_uid(byte arr[], size_t size, String& str) {
  for (size_t i = 0; i < size; i++) {
    str.concat(String(arr[i] < 0x10 ? "0" : " "));
    str.concat(String(arr[i], HEX));
  }
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


void setup_reader() {
  add_peripheral(RFID_CS);
  enable_peripheral(RFID_CS);
  for (byte i = 0; i < 6; i++) {
    key.keyByte[i] = 0xFF;
  }
  mfrc522.PCD_Init();    // Init MFRC522 board.
  disable_peripheral(RFID_CS);
  
}

void poll_reader() {
  enable_peripheral(RFID_CS);
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




#endif  // RFID_READER_H