#include <iostream>
#include <pngwriter.h>

int main(int, char**) {
  pngwriter png(300, 300, 0, "test.png");

  for (int y = 0; y <= 300; ++y)
    png.plot(y, y, 1.0, 1.0, 1.0);

  png.close();
}
