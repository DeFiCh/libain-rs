//
// Created by mambisi on 4/26/22.
//

#include <univalue.h>
#include <amount.h>
#include <string>
#include <masternodes/masternodes.h>
#include <util/strencodings.h>

std::string tokenAmountString(CTokenAmount const& amount) {
    const auto token = pcustomcsview->GetToken(amount.nTokenId);
    const auto valueString = GetDecimaleString(amount.nValue);
    return valueString + "@" + token->CreateSymbolKey(amount.nTokenId);
}

UniValue AmountsToJSON(TAmounts const & diffs) {
    UniValue obj(UniValue::VARR);

    for (auto const & diff : diffs) {
        obj.push_back(tokenAmountString({diff.first, diff.second}));
    }
    return obj;
}

std::string ScriptToString(CScript const& script) {
    CTxDestination dest;
    if (!ExtractDestination(script, dest)) {
        return script.GetHex();
    }
    return EncodeDestination(dest);
}

CAmount AmountFromValue(const UniValue& value)
{
    if (!value.isNum() && !value.isStr())
        throw std::runtime_error("Amount is not a number or string");
    CAmount amount;
    if (!ParseFixedPoint(value.getValStr(), 8, &amount))
        throw std::runtime_error("Invalid amount");
    if (!MoneyRange(amount))
        throw std::runtime_error("Amount out of range");
    return amount;
}

// Found in src/masternodes/rpc_oracles.cpp
bool diffInHour(int64_t time1, int64_t time2) {
    constexpr const int64_t SECONDS_PER_HOUR = 3600u;
    return std::abs(time1 - time2) < SECONDS_PER_HOUR;
}

// Found in src/masternodes/rpc_oracles.cpp
ResVal<CAmount> GetAggregatePrice(CCustomCSView& view, const std::string& token, const std::string& currency, uint64_t lastBlockTime) {
    // DUSD-USD always returns 1.00000000
    if (token == "DUSD" && currency == "USD") {
        return ResVal<CAmount>(COIN, Res::Ok());
    }
    arith_uint256 weightedSum = 0;
    uint64_t numLiveOracles = 0, sumWeights = 0;
    view.ForEachOracle([&](const COracleId&, COracle oracle) {
        if (!oracle.SupportsPair(token, currency)) {
            return true;
        }
        for (const auto& tokenPrice : oracle.tokenPrices) {
            if (token != tokenPrice.first) {
                continue;
            }
            for (const auto& price : tokenPrice.second) {
                if (currency != price.first) {
                    continue;
                }
                const auto& pricePair = price.second;
                auto amount = pricePair.first;
                auto timestamp = pricePair.second;
                if (!diffInHour(timestamp, lastBlockTime)) {
                    continue;
                }
                ++numLiveOracles;
                sumWeights += oracle.weightage;
                weightedSum += arith_uint256(amount) * oracle.weightage;
            }
        }
        return true;
    });

    static const uint64_t minimumLiveOracles = Params().NetworkIDString() == CBaseChainParams::REGTEST ? 1 : 2;

    if (numLiveOracles < minimumLiveOracles) {
        return Res::Err("no live oracles for specified request");
    }

    if (sumWeights == 0) {
        return Res::Err("all live oracles which meet specified request, have zero weight");
    }

    ResVal<CAmount> res((weightedSum / sumWeights).GetLow64(), Res::Ok());
    LogPrint(BCLog::LOAN, "%s(): %s/%s=%lld\n", __func__, token, currency, *res.val);
    return res;
}