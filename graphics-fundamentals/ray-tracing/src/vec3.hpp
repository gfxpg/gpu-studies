#pragma once
#include <cmath>

class Vec3 {
 public:
  constexpr Vec3() : x(0), y(0), z(0){};
  constexpr Vec3(float x, float y, float z) : x(x), y(y), z(z){};
  // Use the default copy constructor created by the compiler (without the
  // following line, it is declared implicitly)
  constexpr Vec3(const Vec3& vec) = default;

  constexpr Vec3& operator+=(Vec3 const& rhs) {
    x += rhs.x;
    y += rhs.y;
    z += rhs.z;
    return *this;
  }
  constexpr Vec3& operator-=(Vec3 const& rhs) {
    x -= rhs.x;
    y -= rhs.y;
    z -= rhs.z;
    return *this;
  }
  constexpr Vec3& operator*=(float rhs) {
    x *= rhs;
    y *= rhs;
    z *= rhs;
    return *this;
  }
  constexpr Vec3& operator/=(float rhs) {
    x /= rhs;
    y /= rhs;
    z /= rhs;
    return *this;
  }
  friend constexpr Vec3 operator+(Vec3 const& lhs, Vec3 const& rhs) {
    return Vec3(lhs) += rhs;
  }
  friend constexpr Vec3 operator-(Vec3 const& lhs, Vec3 const& rhs) {
    return Vec3(lhs) -= rhs;
  }
  friend constexpr Vec3 operator*(Vec3 const& lhs, float rhs) {
    return Vec3(lhs) *= rhs;
  }
  friend constexpr Vec3 operator*(float lhs, Vec3 const& rhs) {
    return Vec3(rhs) *= lhs;
  }
  friend constexpr Vec3 operator/(Vec3 const& lhs, float rhs) {
    return Vec3(lhs) /= rhs;
  }

  constexpr float dot(Vec3 const& rhs) const {
    return x * rhs.x + y * rhs.y + z * rhs.z;
  }

  constexpr Vec3 eltwise_mul(Vec3 const& rhs) const {
    return Vec3(x * rhs.x, y * rhs.y, z * rhs.z);
  }

  constexpr Vec3 operator-() const { return Vec3(-x, -y, -z); }
  constexpr float length() const { return std::sqrt(x * x + y * y + z * z); }
  constexpr float squared_length() const { return x * x + y * y + z * z; }
  constexpr Vec3 unit_vector() const { return *this / length(); }

  union {
    struct {
      float r, g, b;
    };
    struct {
      float x, y, z;
    };
  };
};
