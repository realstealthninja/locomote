#include "mfrc.h"
#include "web.h"

Reader reader{};
void setup() {
  // put your setup code here, to run once:
  Serial.begin(115200);
  reader.setup();
  web_setup();
}

void loop() {
  // put your main code here, to run repeatedly:
  reader.loop();
  web_loop();
}
