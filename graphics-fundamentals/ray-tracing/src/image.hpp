#pragma once
#include <string>
#include <fstream>
#include "vec3.hpp"

class Image {
 public:
  Image(int width, int height, std::string filename)
      : width(width),
        height(height),
        buffer(width * height * 3),
        filename(filename) {}

  inline void set_pixel(int x, int y, Vec3 color) {
    y = height - 1 - y; // ppm starts from the upper left corner
    buffer[0 + (y * width + x) * 3] = static_cast<unsigned char>(255.0 * color.r);
    buffer[1 + (y * width + x) * 3] = static_cast<unsigned char>(255.0 * color.g);
    buffer[2 + (y * width + x) * 3] = static_cast<unsigned char>(255.0 * color.b);
  }

  void write_binary_ppm() const {
    std::ofstream out(filename, std::ios::binary);
    out << "P6\n" << width << " " << height << "\n255\n";
    out.write(reinterpret_cast<const char*>(&buffer[0]), buffer.size() * sizeof(unsigned char));
  }

 private:
  int width, height;
  std::vector<unsigned char> buffer;
  std::string filename;
};
