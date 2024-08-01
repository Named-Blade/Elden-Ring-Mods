#pragma once

#include <inicpp.h>
#include <modUtils.hpp>
#include <tuple>

using namespace ModUtils;

ini::IniFile myIni;

inline std::string operator"" _s(const char* str, std::size_t) {
    return std::string(str);
}

bool hasSectionAndKey(std::string section, std::string key){
    for(const auto &sectionPair : myIni){
        if (sectionPair.first == section) {
            for(const auto &fieldPair : sectionPair.second){
                if (fieldPair.first == key){
                    return true;
                }
            }
        }
    }
    return false;
}

template <typename T>
void getConfigVal(T& value, std::string section, std::string key) {
    try {
        if (hasSectionAndKey(section, key)){
            value = myIni[section][key].as<T>();
        } else {
            myIni[section][key] = value;
        }
    } catch (std::exception e){
        myIni[section][key] = value;
    }
}

#define LVALUE_AND_RVALUE_PAIRS \
    replacePairs(std::string&, std::string&) \
    replacePairs(std::string&, std::string&&) \
    replacePairs(std::string&&, std::string&) \
    replacePairs(std::string&&, std::string&&) \

#define replacePairs(field1, field2) \
    template <typename T, typename... Rest> \
    void getConfigVals(std::tuple<T&, field1, field2> t,Rest&&... rest) { \
        T& value = std::get<0>(t); \
        std::string section = std::get<1>(t); \
        std::string key = std::get<2>(t); \
        getConfigVal(value,section,key); \
        if constexpr (sizeof...(rest) > 0) { \
            getConfigVals(std::forward<Rest>(rest)...); \
        } \
    }
LVALUE_AND_RVALUE_PAIRS

template <typename... Args>
void readConfig(Args&&... args) {
    myIni.load(GetCurrentModPath() + "/" + GetCurrentModName() + ".ini");
    getConfigVals(std::forward<Args>(args)...);
    myIni.save(GetCurrentModPath() + "/" + GetCurrentModName() + ".ini");
}