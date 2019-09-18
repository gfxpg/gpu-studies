#include "vendor/dispatch.hpp"

#define cimg_display 0
#include "vendor/CImg.h"

using namespace amd::dispatch;

#define CODE_OBJECT "test.co"
#define INPUT_IMAGE "kabanchik.bmp"
#define OUTPUT_IMAGE "kabanchik_out.bmp"

class TestDispatch : public Dispatch {
 public:
  TestDispatch(int argc, const char** argv) : Dispatch(argc, argv) {}

  bool SetupCodeObject() override { return LoadCodeObjectFromFile(CODE_OBJECT); }

  bool Setup() override {
    _img = cimg_library::CImg<float>(INPUT_IMAGE);
    auto buffer_size = _img.width() * _img.height() * 3 * sizeof(float);

    if (!AllocateKernarg(buffer_size)) return false;
    _out = AllocateBuffer(buffer_size);

    for (int x = 0; x < _img.width(); ++x)
    for (int y = 0; y < _img.height(); ++y) {
      _out->Ptr<float>()[(x + y * _img.width()) * 3 + 0] = _img(x, y, 0, 0);
      _out->Ptr<float>()[(x + y * _img.width()) * 3 + 1] = _img(x, y, 0, 1);
      _out->Ptr<float>()[(x + y * _img.width()) * 3 + 2] = _img(x, y, 0, 2);
    }

    Kernarg(_out);
    SetGridSize(_img.width(), _img.height());
    SetWorkgroupSize(2, 2);
    return true;
  }

  bool Verify() override {
    if (!CopyFrom(_out)) {
      output << "Error: failed to copy from local" << std::endl;
      return false;
    }

    for (int x = 0; x < _img.width(); ++x)
    for (int y = 0; y < _img.height(); ++y) {
      _img(x, y, 0, 0) = _out->Data<float>((x + y * _img.width()) * 3 + 0);
      _img(x, y, 0, 1) = _out->Data<float>((x + y * _img.width()) * 3 + 1);
      _img(x, y, 0, 2) = _out->Data<float>((x + y * _img.width()) * 3 + 2);
    }

    _img.save(OUTPUT_IMAGE);

    return true;
  }

 private:
  Buffer* _out;
  cimg_library::CImg<float> _img;
};

int main(int argc, const char** argv) {
  return TestDispatch(argc, argv).RunMain();
}
