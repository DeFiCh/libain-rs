#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

using DctId = uint32_t;

struct PoolPair {
  DctId token_a;
  DctId token_b;
  DctId commission;
  int64_t reserve_a;
  int64_t reserve_b;
  int64_t total_liquidity;
  int64_t block_commission_a;
  int64_t block_commission_b;
};

struct TokenAmount {
  DctId token_id;
  int64_t amount;
};

struct PoolPrice {
  int64_t integer;
  int64_t fraction;
};

extern "C" {

int64_t ainrt_execute_dex_swap(const char *dex_module_file_path,
                               PoolPair *poolpair,
                               const TokenAmount *token_in,
                               const PoolPrice *max_price,
                               bool post_bayfront_gardens);

} // extern "C"
