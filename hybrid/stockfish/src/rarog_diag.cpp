/*
  Rarog Phase-4 differential diagnostics for the Stage-1 oracle.
  See rarog_diag.h. Not upstream Stockfish.
*/

#include "rarog_diag.h"

#ifdef RAROG_DIAG

    #include <iostream>

    #include "misc.h"

namespace RarogDiag {

    #define RAROG_DIAG_DEFINE(name) std::atomic<std::uint64_t> name{0};
RAROG_DIAG_COUNTERS(RAROG_DIAG_DEFINE)
    #undef RAROG_DIAG_DEFINE

void reset() {
    #define RAROG_DIAG_RESET(name) name.store(0, std::memory_order_relaxed);
    RAROG_DIAG_COUNTERS(RAROG_DIAG_RESET)
    #undef RAROG_DIAG_RESET
}

void dump() {
    // One line per counter, name emitted verbatim. The Rarog side emits the
    // identical shape, so a differential run is a plain textual join on `name`.
    #define RAROG_DIAG_DUMP(name) \
        sync_cout << "info string diag " #name " " \
                  << name.load(std::memory_order_relaxed) << sync_endl;
    RAROG_DIAG_COUNTERS(RAROG_DIAG_DUMP)
    #undef RAROG_DIAG_DUMP
}

}  // namespace RarogDiag

#endif  // RAROG_DIAG
