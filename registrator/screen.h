#define LGFX_AUTODETECT

#include <LovyanGFX.hpp>
#include <LGFX_AUTODETECT.hpp>

static LGFX lcd;
static LGFX_Sprite sprite(&lcd);




void lcd_setup() {
  lcd.init();
  lcd.setRotation(1);
  lcd.setColorDepth(24);
  lcd.setBrightness(180);
  lcd.drawFastVLine(2, 0, 100, lcd.color888(255, 0, 0));
  lcd.drawString("hello world", 50, 100, 4);
  
}


void lcd_loop() {
  
  lcd.display();
}