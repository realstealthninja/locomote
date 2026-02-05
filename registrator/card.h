#ifndef CARD_H
#define CARD_H

#include <vector>
#include <MFRC522v2.h>


class Card {
private:
  std::vector<std::byte> uid;

public:
  Card() {

  };

  void write_data(const std::vector<std::byte>& bytes) {

  }

  void read_data(std::vector<std::byte>& bytes) {

  }



};


#endif // CARD_H