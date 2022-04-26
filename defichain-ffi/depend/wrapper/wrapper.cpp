//
// Created by mambisi on 4/26/22.
//

#include <univalue.h>
#include <amount.h>
#include <string>
#include <masternodes/masternodes.h>

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