
#include "reader.h"

void setup() {
  Serial.begin(115200);
  while (!Serial); 

  setup_reader();

}

void loop() {
  poll_reader();

  

  delay(2000);
}