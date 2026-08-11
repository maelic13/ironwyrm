/*
  Rarog/Stockfish HCE hybrid adapter.

  This file is distributed under the GNU General Public License version 3
  or (at your option) any later version.
*/

#include "hybrid_eval.h"

#include <algorithm>
#include <cstdlib>
#include <cstdint>
#include <cstring>
#include <limits>
#include <sstream>

#include "position.h"

#ifdef _WIN32
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <windows.h>
#endif

namespace HybridEval {
namespace {

constexpr std::uint32_t ExpectedAbiVersion = 1;
using AbiVersionFn = std::uint32_t (*)();
using EvaluateFn = std::int32_t (*)(const std::uint64_t*, std::uint8_t,
                                    std::uint8_t, std::uint8_t);

EvaluateFn evaluator = nullptr;
bool useRarogHce = true;

#ifdef _WIN32
HMODULE library = nullptr;

std::string windows_error(const char* operation) {
  std::ostringstream out;
  out << operation << " failed (Windows error " << GetLastError() << ')';
  return out.str();
}

template<typename Function>
Function load_function(HMODULE module, const char* name) {
  const FARPROC address = GetProcAddress(module, name);
  static_assert(sizeof(Function) == sizeof(address), "unexpected Windows function-pointer size");
  Function function;
  std::memcpy(&function, &address, sizeof(function));
  return function;
}
#endif

} // namespace

bool initialize(std::string& error) {
  if (evaluator)
      return true;

#ifdef _WIN32
  wchar_t executablePath[MAX_PATH];
  const DWORD length = GetModuleFileNameW(nullptr, executablePath, MAX_PATH);
  if (!length || length == MAX_PATH) {
      error = windows_error("GetModuleFileNameW");
      return false;
  }

  wchar_t* separator = executablePath + length;
  while (separator != executablePath && separator[-1] != L'\\' && separator[-1] != L'/')
      --separator;
  const wchar_t dllName[] = L"rarog_hce.dll";
  if (separator - executablePath + sizeof(dllName) / sizeof(dllName[0]) > MAX_PATH) {
      error = "the executable path is too long to locate rarog_hce.dll";
      return false;
  }
  std::copy(dllName, dllName + sizeof(dllName) / sizeof(dllName[0]), separator);

  library = LoadLibraryW(executablePath);
  if (!library) {
      error = windows_error("loading rarog_hce.dll beside the executable");
      return false;
  }

  const auto abi = load_function<AbiVersionFn>(library, "rarog_hce_abi_version");
  evaluator = load_function<EvaluateFn>(library, "rarog_hce_evaluate");
  if (!abi || !evaluator) {
      error = windows_error("resolving the Rarog HCE ABI");
      evaluator = nullptr;
      return false;
  }
  if (abi() != ExpectedAbiVersion) {
      std::ostringstream out;
      out << "rarog_hce.dll ABI mismatch: expected " << ExpectedAbiVersion
          << ", received " << abi();
      error = out.str();
      evaluator = nullptr;
      return false;
  }
  return true;
#else
  error = "this Stage 1 adapter currently supports Windows only";
  return false;
#endif
}

bool enabled() {
  return useRarogHce;
}

void set_enabled(bool value) {
  useRarogHce = value;
}

Value evaluate(const Position& pos) {
  std::uint64_t pieces[12];
  for (Color color : {WHITE, BLACK})
      for (PieceType piece = PAWN; piece <= KING; ++piece)
          pieces[color * 6 + piece - 1] = pos.pieces(color, piece);

  const int rule50 = std::min(pos.rule50_count(), 100);
  const auto raw = evaluator(pieces,
                             static_cast<std::uint8_t>(pos.side_to_move()),
                             static_cast<std::uint8_t>(pos.castling_rights(WHITE)
                                                     | pos.castling_rights(BLACK)),
                             static_cast<std::uint8_t>(rule50));
  if (raw == std::numeric_limits<std::int32_t>::min())
      std::abort(); // A corrupt position must never silently enter the search.

  // Rarog scores in centipawns. Stockfish's search unit is PawnValueEg/100.
  const std::int64_t numerator = static_cast<std::int64_t>(raw) * int(PawnValueEg);
  const std::int64_t scaled = numerator >= 0 ? (numerator + 50) / 100
                                             : (numerator - 50) / 100;
  return Value(std::max<std::int64_t>(-VALUE_KNOWN_WIN + 1,
               std::min<std::int64_t>(VALUE_KNOWN_WIN - 1, scaled)));
}

} // namespace HybridEval
