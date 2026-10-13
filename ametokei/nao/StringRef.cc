//===-- StringRef.cpp - Lightweight String References ---------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "nao/StringRef.h"

#include "rapidhash.h"

using namespace ametokei;

int ametokei::ascii_strncasecmp(StringRef LHS, StringRef RHS) {
  assert(LHS.size() == RHS.size() &&
         "2 these string_view don't have a same size");
  for (size_t I = 0; I < RHS.size(); I++) {
    auto LC = LHS[I];
    auto RC = RHS[I];

    unsigned char LHC = ToLower(LC);
    unsigned char RHC = ToLower(RC);
    if (LHC != RHC)
      return LHC < RHC ? -1 : 1;
  }
  return 0;
}

uint64_t ametokei::StringRef::hash_value() const {
  return rapidhash(data(), size());
}
