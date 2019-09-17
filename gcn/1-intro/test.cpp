#include "vendor/dispatch.hpp"

using namespace amd::dispatch;

class TestDispatch : public Dispatch {
 public:
  TestDispatch(int argc, const char** argv) : Dispatch(argc, argv) {}

  bool SetupCodeObject() override { return LoadCodeObjectFromFile("test.co"); }

  bool Setup() override {
    if (!AllocateKernarg(1024)) return false;

    _out = AllocateBuffer(1024);
    Kernarg(_out);
    SetGridSize(1);
    SetWorkgroupSize(1);
    return true;
  }

  bool Verify() override {
    if (!CopyFrom(_out)) {
      output << "Error: failed to copy from local" << std::endl;
      return false;
    }
    if (((const float*)_out->SystemPtr())[0] != 3.14159f) {
      output << "Error: validation failed: got "
             << (((const float*)_out->SystemPtr())[0]) << " expected " << 3.14159
             << std::endl;
      return false;
    }
    return true;
  }

 private:
  Buffer* _out;
};

int main(int argc, const char** argv) {
  return TestDispatch(argc, argv).RunMain();
}
