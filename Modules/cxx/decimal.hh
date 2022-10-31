#pragma once

#define TODO() static_assert(false, "TODO")

#include "byte_len.ii"
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
// uint32 - 정확히 32비트
// uint32_fast - 32비트 이상, 가장 빠른것
// uint32_least - 32비트 이상

namespace evh {

namespace {
/// @brief 숫자의 부호
enum Signal : uint_fast8_t
{
  /// @brief 숫자가 0이다.
  zero     = 0,
  /// @brief 숫자가 +이다.
  positive = 1,
  /// @brief 숫자가 -이다.
  negetive = 2,
};
}

/**
 * @brief 큰 자료형 숫자를 저장하는 클래스
 * @note 32비트 정수형은 2_147_483_647까지 저장할 수 있는것을 기반으로, 한 글자 당 9자리를 저장해 사용한다.
 */
class Decimal
{
private:
  static inline size_t        byte_len(const char* str) noexcept { return byte_len_asm(str); }
  /// @brief 빠른 uint32 파싱
  /// @param str 변환할 문자열
  /// @return 파싱된 문자열
  static inline uint_fast32_t atoui32(const char* str) noexcept
  {
    uint_fast32_t value = 0;
    char          c;
    while (*str) {
      c = *str;
      ++str;
      value *= 10;
      value += c & 0xf;
    }
    return value;
  }

protected:
  /// @brief 숫자의 부호
  Signal    signal;
  /// @brief data 배열의 길이
  uint64_t  data_length = 0;
  /// @brief 데이터가 들어있는 배열
  uint32_t* data        = nullptr;

public:
  /// @brief 데이터 0으로 초기화
  Decimal() noexcept
  {
    this->signal      = Signal::zero;
    this->data_length = 0;
    this->data        = nullptr;
  }

  Decimal(const char* str) noexcept
  {
    if (str == nullptr || str[0] == '\0') {
      /* 데이터가 0일때 */
      // 포인터가 비어있거나, 배열이 비어있으면 빈 데이터로 처리한다.

      this->signal      = Signal::zero;
      this->data_length = 0;
      this->data        = nullptr;
    } else if (str[0] == '-') {
      /* 데이터가 음수일때 */
      // 첫 글자가 -일경우 음수로 처리한다.

    } else {
      /* 데이터가 양수일때 */
      // 그 외는 양수로 처리한다.

      // 양수 설정
      this->signal      = Signal::positive;
      // 입력 데이터의 글자수
      auto len          = this->byte_len(str);
      // data마다 9자릿수를 저장한다.
      this->data_length = (len / 9) + 1;
      // data 배열 할당
      this->data        = new uint32_t[this->data_length];

      // 버퍼 설정
      char          buf[10]    = { 0 };
      uint_fast32_t data_index = 0;
      size_t        buf_index  = len - 9;
      // 데이터 길이가 9자리씩 끊어서 저장
      while (buf_index > 0) {
        // 뒤에서부터 순차적으로 9자리씩 버퍼에 옮긴 후
        strncpy(buf, str + buf_index, 9);
        // 숫자형으로 파싱하여 저장
        this->data[data_index] = this->atoui32(buf);
        // 이후 인덱스 이동
        buf_index -= 9;
        data_index += 1;
      }
      // 남은 데이터 저장
      strncpy(buf, str, buf_index + 9);
      buf[buf_index + 9]     = '\0';
      this->data[data_index] = this->atoui32(buf);
    }
  }

  /// @brief 데이터 복사
  /// @param other 다른 Decimal 객체
  Decimal(const Decimal& other) noexcept
  {
    /* 데이터 복사 */
    this->signal      = other.signal;
    this->data_length = other.data_length;
    this->data        = new uint32_t[this->data_length];
    for (uint64_t i = 0; i < this->data_length; i++)
      this->data[i] = other.data[i];
  }

  /// @brief 이동연산자, 데이터를 복사하고 other의 데이터를 삭제한다.
  /// @param other 다른 Decimal 객체
  Decimal(Decimal&& other) noexcept
  {
    /* 데이터 복사 */
    this->signal      = other.signal;
    this->data_length = other.data_length;
    this->data        = other.data;
    /* other 삭제 */
    other.data        = nullptr;
    other.data_length = 0;
    other.signal      = Signal::zero;
  }

  /// @brief 삭제연산자
  ~Decimal() noexcept
  {
    if (this->data != nullptr)
      delete[] this->data;
  }

  inline void operator=(const Decimal& other) noexcept
  {
    /* 데이터 복사 */
    this->signal      = other.signal;
    this->data_length = other.data_length;
    this->data        = new uint32_t[this->data_length];
    for (uint64_t i = 0; i < this->data_length; i++)
      this->data[i] = other.data[i];
  }

  inline Decimal operator+(const uint32_t& data) noexcept { TODO(); }

  inline const std::string to_string() const noexcept
  {
    if (this->data_length == 0) {
      return "0";
    }
  }
};

}