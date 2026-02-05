#ifndef SPI_BUS_H
#define SPI_BUS_H

#include <cstdint>
#include <vector>

std::vector<uint32_t> spi_ss_pins{};

void enable_peripheral(uint32_t pin) {
  for(const auto& spi_pin : spi_ss_pins) {
    if (spi_pin == pin) {
      digitalWrite(pin, HIGH);
    }
    digitalWrite(pin, LOW);
  }
}

void disable_peripheral(uint32_t pin) {
  digitalWrite(pin, LOW);
}

void add_peripheral(uint32_t pin) {
  spi_ss_pins.push_back(pin);
}


#endif SPI_BUS_H