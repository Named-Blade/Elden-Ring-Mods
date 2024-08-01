#pragma once

#include <inicpp.h>
#include <modUtils.hpp>
#include <tuple>

using namespace ModUtils;

ini::IniFile myIni;

template <typename T>
void getConfigVal(T& value, std::string& section, std::string& name) {
    value = myIni[section][name].as<T>();
}

template <typename T, typename... Rest>
void getConfigVals(std::tuple<T&, std::string&, std::string&> t,Rest... rest) {
    T& value = std::get<0>(t);
    std::string& section = std::get<1>(t);
    std::string& name = std::get<2>(t);
    getConfigVal(value,section,name);
    if constexpr (sizeof...(rest) > 0) {
        getConfigVals(rest...);
    }
}

template <typename... Args>
void readConfig(Args... args) {
    myIni.load(GetCurrentModPath() + "/" + GetCurrentModName() + ".ini");
    getConfigVals(args...);
}