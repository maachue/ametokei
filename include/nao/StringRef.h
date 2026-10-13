//===- StringRef.h - Constant String Reference Wrapper ----------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef AMETOKEI_NAO_STRINGREF_H
#define AMETOKEI_NAO_STRINGREF_H

#include <algorithm>
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <functional>
#include <ostream>
#include <string>
#include <string_view>
#include <type_traits>

namespace ametokei {
template <typename T> struct remove_cvref {
  using type = std::remove_cv_t<std::remove_reference_t<T>>;
};

template <typename T> using remove_cvref_t = typename remove_cvref<T>::type;

template <typename Fn> class function_ref;
template <typename Ret, typename... Params> class function_ref<Ret(Params...)> {
  Ret (*callback)(intptr_t callable, Params... params) = nullptr;
  intptr_t callable;

  template <typename Callable>
  static Ret callback_fn(intptr_t callable, Params... params) {
    return (*reinterpret_cast<Callable *>(callable))(
        std::forward<Params>(params)...);
  }

public:
  function_ref() = default;

  template <typename Callable>
  function_ref(
      Callable &&callable,
      // This is not the copy-constructor.
      std::enable_if_t<!std::is_same_v<remove_cvref_t<Callable>, function_ref>>
          * = nullptr,
      // Functor must be callable and return a suitable type.
      std::enable_if_t<std::is_void_v<Ret> ||
                       std::is_convertible_v<decltype(std::declval<Callable>()(
                                                 std::declval<Params>()...)),
                                             Ret>> * = nullptr)
      : callback(callback_fn<std::remove_reference_t<Callable>>),
        callable(reinterpret_cast<intptr_t>(&callable)) {}

  Ret operator()(Params... params) const {
    return callback(callable, std::forward<Params>(params)...);
  }

  explicit operator bool() const { return callback; }

  bool operator==(const function_ref<Ret(Params...)> &Other) const {
    return callable == Other.callable;
  }
};

class StringRef;

inline char ToLower(char x) {
  if ('A' <= x && x <= 'Z')
    return x - 'A' + 'a';
  return x;
}

int ascii_strncasecmp(StringRef LHS, StringRef RHS);

class StringRef {
public:
  static constexpr size_t npos = ~static_cast<size_t>(0);

  using iterator = const char *;
  using const_iterator = const char *;
  using size_type = size_t;
  using value_type = char;
  using reverse_iterator = std::reverse_iterator<iterator>;
  using const_reverse_iterator = std::reverse_iterator<const_iterator>;

  constexpr StringRef() = default;
  constexpr StringRef(std::nullptr_t) = delete;

  constexpr StringRef(const char *S)
      : StringRef(S ? std::string_view(S) : std::string_view()) {}
  constexpr StringRef(std::string_view Str)
      : Data_(Str.data()), Size_(Str.size()) {}

  constexpr StringRef(std::string const &S)
      : Data_(S.data()), Size_(S.size()) {}
  constexpr StringRef(const char *S, size_t Size) : Data_(S), Size_(Size) {}

  [[nodiscard]] uint64_t hash_value() const;

  /// \name iterators
  /// \{

  [[nodiscard]] iterator begin() const { return data(); }
  [[nodiscard]] iterator end() const { return data() + size(); }

  [[nodiscard]] reverse_iterator rbegin() const {
    return std::make_reverse_iterator(end());
  }
  [[nodiscard]] reverse_iterator rend() const {
    return std::make_reverse_iterator(begin());
  }

  /// \}
  /// \name String Operations
  /// \{

  [[nodiscard]] constexpr const char *data() const { return Data_; }
  [[nodiscard]] constexpr size_t size() const { return Size_; }

  [[nodiscard]] constexpr bool empty() const { return size() == 0; }

  [[nodiscard]] constexpr char front() const {
    assert(!empty());
    return *data();
  }

  [[nodiscard]] constexpr char back() const {
    assert(!empty());
    return data()[size() - 1];
  }

  [[nodiscard]] std::string str() const { return {data(), size()}; }

  [[nodiscard]] int compare(StringRef RHS) const {
    if (int Res =
            compareMemory(data(), RHS.data(), std::min(size(), RHS.size())))
      return Res < 0 ? -1 : 1;

    if (size() == RHS.size())
      return 0;
    return size() < RHS.size() ? -1 : 1;
  }

  [[nodiscard]] int compare_insensitive(StringRef RHS) const;

  constexpr StringRef nonEmptyOr(StringRef Str) const {
    assert(!Str.empty() && "nonEmptyOr should not have an empty default!");
    return empty() ? Str : *this;
  }

  /// \}
  /// \name Operator Overloads
  /// \{

  constexpr char operator[](size_t Idx) const {
    assert(Idx < size() && "Indvalid index!");
    return data()[Idx];
  }

  template <typename T>
  std::enable_if_t<std::is_same_v<T, std::string>, StringRef> &
  operator=(T &&S) = delete;

  constexpr operator std::string_view() const { return {data(), size()}; }

  /// \}
  /// \name String Predicates
  /// \{

  [[nodiscard]] bool starts_with(StringRef Prefix) const {
    return size() >= Prefix.size() &&
           compareMemory(data(), Prefix.data(), Prefix.size()) == 0;
  }

  [[nodiscard]] bool starts_with(char Prefix) const {
    return !empty() && front() == Prefix;
  }

  [[nodiscard]] bool starts_with_insensitive(StringRef Prefix) const;

  [[nodiscard]] bool ends_with(StringRef Suffix) const {
    return size() >= Suffix.size() &&
           compareMemory(end() - Suffix.size(), Suffix.data(), Suffix.size()) ==
               0;
  }

  [[nodiscard]] bool ends_with(char Suffix) const {
    return !empty() && back() == Suffix;
  }

  [[nodiscard]] bool ends_with_insensitive(StringRef Suffix) const;

  /// \}
  /// \name String Searching
  /// \{

  [[nodiscard]] size_t find(char C, size_t From = 0) const {
    return std::string_view{*this}.find(C, From);
  }

  [[nodiscard]] size_t find_insensitive(char C, size_t From = 0) const;

  [[nodiscard]] size_t find_if(function_ref<bool(char)> F,
                               size_t From = 0) const {
    StringRef S = drop_front(From);
    while (!S.empty()) {
      if (F(S.front()))
        return size() - S.size();
      S = S.drop_front();
    }

    return npos;
  }

  [[nodiscard]] size_t find_if_not(function_ref<bool(char)> F,
                                   size_t From = 0) const {
    return find_if(std::not_fn(F), From);
  }

  [[nodiscard]] size_t rfind_if(function_ref<bool(char)> F,
                                size_t End = npos) const {
    size_t I = std::min(End, size());
    while (I) {
      --I;
      if (F(data()[I]))
        return I;
    }

    return npos;
  }

  [[nodiscard]] size_t rfind_if_not(function_ref<bool(char)> F,
                                    size_t End = npos) const {
    return rfind_if(std::not_fn(F), End);
  }

  [[nodiscard]] size_t find(StringRef Str, size_t From = 0) const;

  [[nodiscard]] size_t find_insensitive(StringRef Str, size_t From = 0) const;

  [[nodiscard]] size_t rfind(char C, size_t From = npos) const {
    size_t I = std::min(From, size());

    while (I) {
      --I;
      if (data()[I] == C)
        return I;
    }

    return npos;
  }

  [[nodiscard]] size_t rfind_insensitive(char C, size_t From = npos) const;

  [[nodiscard]] size_t rfind(StringRef Str) const;

  [[nodiscard]] size_t rfind_insensitive(StringRef Str) const;

  [[nodiscard]] size_t find_first_of(char C, size_t From = 0) const {
    return find(C, From);
  }

  [[nodiscard]] size_t find_first_of(StringRef Chars, size_t From = 0) const;

  [[nodiscard]] size_t find_first_not_of(char C, size_t From = 0) const {
    return std::string_view(*this).find_first_not_of(C, From);
  }

  [[nodiscard]] size_t find_first_not_of(StringRef Chars,
                                         size_t From = 0) const;

  [[nodiscard]] size_t find_last_of(char C, size_t From = npos) const {
    return rfind(C, From);
  }

  [[nodiscard]] size_t find_last_of(StringRef Chars, size_t From = npos) const;

  [[nodiscard]] size_t find_last_not_of(char C, size_t From = npos) const;

  [[nodiscard]] size_t find_last_not_of(StringRef Chars,
                                        size_t From = npos) const;

  [[nodiscard]] bool contains(StringRef Other) const {
    return find(Other) != npos;
  }

  [[nodiscard]] bool contains(char C) const { return find_first_of(C) != npos; }

  [[nodiscard]] bool contains_insensitive(StringRef Other) const {
    return find_insensitive(Other) != npos;
  }

  [[nodiscard]] bool contains_insensitive(char C) const {
    return find_insensitive(C) != npos;
  }

  /// \}
  /// \name Helpful Algorithms
  /// \{

  [[nodiscard]] size_t count(char C) const {
    size_t Count = 0;
    for (size_t I = 0; I != size(); ++I)
      if (data()[I] == C)
        ++Count;

    return Count;
  }

  [[nodiscard]] size_t count(StringRef Str) const;

  [[nodiscard]] std::string lower() const;
  [[nodiscard]] std::string upper() const;

  /// \}
  /// \name Substring Operations
  /// \{

  [[nodiscard]] constexpr StringRef substr(size_t Start,
                                           size_t N = npos) const {
    Start = std::min(Start, size());
    return {data() + Start, std::min(N, size() - Start)};
  }

  /// Return a StringRef equal to 'this' but with only the first \p N
  /// elements remaining.  If \p N is greater than the length of the
  /// string, the entire string is returned.
  [[nodiscard]] StringRef take_front(size_t N = 1) const {
    if (N >= size())
      return *this;
    return drop_back(size() - N);
  }

  /// Return a StringRef equal to 'this' but with only the last \p N
  /// elements remaining.  If \p N is greater than the length of the
  /// string, the entire string is returned.
  [[nodiscard]] StringRef take_back(size_t N = 1) const {
    if (N >= size())
      return *this;
    return drop_front(size() - N);
  }

  /// Return the longest prefix of 'this' such that every character
  /// in the prefix satisfies the given predicate.
  [[nodiscard]] StringRef take_while(function_ref<bool(char)> F) const {
    return substr(0, find_if_not(F));
  }

  /// Return the longest prefix of 'this' such that no character in
  /// the prefix satisfies the given predicate.
  [[nodiscard]] StringRef take_until(function_ref<bool(char)> F) const {
    return substr(0, find_if(F));
  }

  /// Return a StringRef equal to 'this' but with the first \p N elements
  /// dropped.
  [[nodiscard]] StringRef drop_front(size_t N = 1) const {
    assert(size() >= N && "Dropping more elements than exist");
    return substr(N);
  }

  /// Return a StringRef equal to 'this' but with the last \p N elements
  /// dropped.
  [[nodiscard]] StringRef drop_back(size_t N = 1) const {
    assert(size() >= N && "Dropping more elements than exist");
    return substr(0, size() - N);
  }

  /// Return a StringRef equal to 'this', but with all characters satisfying
  /// the given predicate dropped from the beginning of the string.
  [[nodiscard]] StringRef drop_while(function_ref<bool(char)> F) const {
    return substr(find_if_not(F));
  }

  /// Return a StringRef equal to 'this', but with all characters not
  /// satisfying the given predicate dropped from the beginning of the string.
  [[nodiscard]] StringRef drop_until(function_ref<bool(char)> F) const {
    return substr(find_if(F));
  }

  /// Returns true if this StringRef has the given prefix and removes that
  /// prefix.
  bool consume_front(char Prefix) {
    if (!starts_with(Prefix))
      return false;

    *this = drop_front();
    return true;
  }

  /// Returns true if this StringRef has the given prefix and removes that
  /// prefix.
  bool consume_front(StringRef Prefix) {
    if (!starts_with(Prefix))
      return false;

    *this = substr(Prefix.size());
    return true;
  }

  /// Returns true if this StringRef has the given prefix, ignoring case,
  /// and removes that prefix.
  bool consume_front_insensitive(StringRef Prefix) {
    if (!starts_with_insensitive(Prefix))
      return false;

    *this = substr(Prefix.size());
    return true;
  }

  /// Returns true if this StringRef has the given suffix and removes that
  /// suffix.
  bool consume_back(StringRef Suffix) {
    if (!ends_with(Suffix))
      return false;

    *this = substr(0, size() - Suffix.size());
    return true;
  }

  /// Returns true if this StringRef has the given suffix, ignoring case,
  /// and removes that suffix.
  bool consume_back_insensitive(StringRef Suffix) {
    if (!ends_with_insensitive(Suffix))
      return false;

    *this = substr(0, size() - Suffix.size());
    return true;
  }

  /// Return a reference to the substring from [Start, End).
  ///
  /// \param Start The index of the starting character in the substring; if
  /// the index is npos or greater than the length of the string then the
  /// empty substring will be returned.
  ///
  /// \param End The index following the last character to include in the
  /// substring. If this is npos or exceeds the number of characters
  /// remaining in the string, the string suffix (starting with \p Start)
  /// will be returned. If this is less than \p Start, an empty string will
  /// be returned.
  [[nodiscard]] StringRef slice(size_t Start, size_t End) const {
    Start = std::min(Start, size());
    End = std::clamp(End, Start, size());
    return StringRef(data() + Start, End - Start);
  }

  /// Split into two substrings around the first occurrence of a separator
  /// character.
  ///
  /// If \p Separator is in the string, then the result is a pair (LHS, RHS)
  /// such that (*this == LHS + Separator + RHS) is true and RHS is
  /// maximal. If \p Separator is not in the string, then the result is a
  /// pair (LHS, RHS) where (*this == LHS) and (RHS == "").
  ///
  /// \param Separator The character to split on.
  /// \returns The split substrings.
  [[nodiscard]] std::pair<StringRef, StringRef> split(char Separator) const {
    return split(StringRef(&Separator, 1));
  }

  /// Split into two substrings around the first occurrence of a separator
  /// string.
  ///
  /// If \p Separator is in the string, then the result is a pair (LHS, RHS)
  /// such that (*this == LHS + Separator + RHS) is true and RHS is
  /// maximal. If \p Separator is not in the string, then the result is a
  /// pair (LHS, RHS) where (*this == LHS) and (RHS == "").
  ///
  /// \param Separator - The string to split on.
  /// \return - The split substrings.
  [[nodiscard]] std::pair<StringRef, StringRef>
  split(StringRef Separator) const {
    size_t Idx = find(Separator);
    if (Idx == npos)
      return {*this, StringRef()};
    return {slice(0, Idx), substr(Idx + Separator.size())};
  }

  /// Split into two substrings around the last occurrence of a separator
  /// string.
  ///
  /// If \p Separator is in the string, then the result is a pair (LHS, RHS)
  /// such that (*this == LHS + Separator + RHS) is true and RHS is
  /// minimal. If \p Separator is not in the string, then the result is a
  /// pair (LHS, RHS) where (*this == LHS) and (RHS == "").
  ///
  /// \param Separator - The string to split on.
  /// \return - The split substrings.
  [[nodiscard]] std::pair<StringRef, StringRef>
  rsplit(StringRef Separator) const {
    size_t Idx = rfind(Separator);
    if (Idx == npos)
      return {*this, StringRef()};
    return {slice(0, Idx), substr(Idx + Separator.size())};
  }

  // /// Split into substrings around the occurrences of a separator string.
  // ///
  // /// Each substring is stored in \p A. If \p MaxSplit is >= 0, at most
  // /// \p MaxSplit splits are done and consequently <= \p MaxSplit + 1
  // /// elements are added to A.
  // /// If \p KeepEmpty is false, empty strings are not added to \p A. They
  // /// still count when considering \p MaxSplit
  // /// An useful invariant is that
  // /// Separator.join(A) == *this if MaxSplit == -1 and KeepEmpty == true
  // ///
  // /// \param A - Where to put the substrings.
  // /// \param Separator - The string to split on.
  // /// \param MaxSplit - The maximum number of times the string is split.
  // /// \param KeepEmpty - True if empty substring should be added.
  // LLVM_ABI void split(SmallVectorImpl<StringRef> &A, StringRef Separator,
  //                     int MaxSplit = -1, bool KeepEmpty = true) const;

  // /// Split into substrings around the occurrences of a separator character.
  // ///
  // /// Each substring is stored in \p A. If \p MaxSplit is >= 0, at most
  // /// \p MaxSplit splits are done and consequently <= \p MaxSplit + 1
  // /// elements are added to A.
  // /// If \p KeepEmpty is false, empty strings are not added to \p A. They
  // /// still count when considering \p MaxSplit
  // /// An useful invariant is that
  // /// Separator.join(A) == *this if MaxSplit == -1 and KeepEmpty == true
  // ///
  // /// \param A - Where to put the substrings.
  // /// \param Separator - The string to split on.
  // /// \param MaxSplit - The maximum number of times the string is split.
  // /// \param KeepEmpty - True if empty substring should be added.
  // LLVM_ABI void split(SmallVectorImpl<StringRef> &A, char Separator,
  //                     int MaxSplit = -1, bool KeepEmpty = true) const;

  /// Split into two substrings around the last occurrence of a separator
  /// character.
  ///
  /// If \p Separator is in the string, then the result is a pair (LHS, RHS)
  /// such that (*this == LHS + Separator + RHS) is true and RHS is
  /// minimal. If \p Separator is not in the string, then the result is a
  /// pair (LHS, RHS) where (*this == LHS) and (RHS == "").
  ///
  /// \param Separator - The character to split on.
  /// \return - The split substrings.
  [[nodiscard]] std::pair<StringRef, StringRef> rsplit(char Separator) const {
    return rsplit(StringRef(&Separator, 1));
  }

  /// Return string with consecutive \p Char characters starting from the
  /// the left removed.
  [[nodiscard]] StringRef ltrim(char Char) const {
    return drop_front(std::min(size(), find_first_not_of(Char)));
  }

  /// Return string with consecutive characters in \p Chars starting from
  /// the left removed.
  [[nodiscard]] StringRef ltrim(StringRef Chars = " \t\n\v\f\r") const {
    return drop_front(std::min(size(), find_first_not_of(Chars)));
  }

  /// Return string with consecutive \p Char characters starting from the
  /// right removed.
  [[nodiscard]] StringRef rtrim(char Char) const {
    return drop_back(size() - std::min(size(), find_last_not_of(Char) + 1));
  }

  /// Return string with consecutive characters in \p Chars starting from
  /// the right removed.
  [[nodiscard]] StringRef rtrim(StringRef Chars = " \t\n\v\f\r") const {
    return drop_back(size() - std::min(size(), find_last_not_of(Chars) + 1));
  }

  /// Return string with consecutive \p Char characters starting from the
  /// left and right removed.
  [[nodiscard]] StringRef trim(char Char) const {
    return ltrim(Char).rtrim(Char);
  }

  /// Return string with consecutive characters in \p Chars starting from
  /// the left and right removed.
  [[nodiscard]] StringRef trim(StringRef Chars = " \t\n\v\f\r") const {
    return ltrim(Chars).rtrim(Chars);
  }

  /// Detect the line ending style of the string.
  ///
  /// If the string contains a line ending, return the line ending character
  /// sequence that is detected. Otherwise return '\n' for unix line endings.
  ///
  /// \return - The line ending character sequence.
  [[nodiscard]] StringRef detectEOL() const {
    size_t Pos = find('\r');
    if (Pos == npos) {
      // If there is no carriage return, assume unix
      return "\n";
    }
    if (Pos + 1 < size() && data()[Pos + 1] == '\n')
      return "\r\n"; // Windows
    if (Pos > 0 && data()[Pos - 1] == '\n')
      return "\n\r"; // You monster!
    return "\r";     // Classic Mac
  }

  /// \}

private:
  const char *Data_ = nullptr;
  size_t Size_ = 0;

  static int compareMemory(const char *Lhs, const char *Rhs, size_t Length) {
    if (Length == 0)
      return 0;
    return ::memcmp(Lhs, Rhs, Length);
  }

  friend bool operator==(StringRef RHS, StringRef LHS) {
    return RHS.compare(LHS);
  }
};

inline std::ostream &operator<<(std::ostream &OS, StringRef S) {
  OS << std::string_view{S};
  return OS;
}
} // namespace ametokei

namespace std {
template <> struct hash<ametokei::StringRef> {
  size_t operator()(const ametokei::StringRef &P) const noexcept {
    return P.hash_value();
  }
};
} // namespace std

#endif
