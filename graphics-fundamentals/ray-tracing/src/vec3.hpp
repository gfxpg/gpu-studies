class Vec3 {
 public:
  // Initializer lists are permitted in constexpr constructors — constructors that _may_ be evaluated during compile time
  constexpr Vec3() : x(0), y(0), z(0){};
  constexpr Vec3(float x, float y, float z) : x(x), y(y), z(z){};
  // Use the default copy constructor created by the compiler (without the following line, it is declared implicitly)
  constexpr Vec3(const Vec3& vec) = default;

  constexpr Vec3& operator+=(Vec3 const& rhs) {
    this->x += rhs.x;
    this->y += rhs.y;
    this->z += rhs.z;
    return *this;
  }
  friend constexpr Vec3 operator+(Vec3 const& lhs, Vec3 const& rhs) {
    return Vec3(lhs) += rhs;
  }

  union {
    struct {
      float r, g, b;
    };
    struct {
      float x, y, z;
    };
  };
};
