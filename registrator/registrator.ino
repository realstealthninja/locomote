#include "reader.h"

RFIDReader reader();

void setup() {
  Serial.begin(115200);
  while (!Serial); 

  reader.setup();

}

void loop() {
  reader.poll();
}