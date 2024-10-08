#include <inicpp.h>
#include "endlessNg.hpp"

std::string getDLLName()
{
	char path[MAX_PATH];
	HMODULE hm = NULL;

	if (GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | 
					GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
					(LPCSTR) &getDLLName, &hm) == 0)
	{
			int ret = GetLastError();
	}
	if (GetModuleFileName(hm, path, sizeof(path)) == 0)
	{
			int ret = GetLastError();
	}
	std::string pathStr(path);
	
	return pathStr.substr(0,pathStr.size()-4);
}

std::string modName = getDLLName();

std::string getDllPath() 
{
  char path[MAX_PATH];
	HMODULE hm = NULL;

	if (GetModuleHandleEx(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | 
					GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
					(LPCSTR) &getDllPath, &hm) == 0)
	{
			int ret = GetLastError();
	}
	if (GetModuleFileName(hm, path, sizeof(path)) == 0)
	{
			int ret = GetLastError();
	}
	std::string pathStr(path);
	
	return pathStr.substr(0,pathStr.size() - (modName.size() + 4));
}

bool strToBool (std::string configStr)
{
	if (configStr == "1"){
		return true;
	} else {
		return false;
	}
}

float strToFloat (std::string configStr)
{
	float configFloat;
	try{
		configFloat = std::stof(configStr);
	} catch (const std::invalid_argument& e) {
		return 1.0;
	} catch (const std::out_of_range& e) {
		return 1.0;
	}
	
	if (configFloat > 1.0){
		return 1.0;
	} else if (configFloat < 0.0){
		return 0.0;
	}
	return configFloat;
}

int strToInt (std::string configStr)
{
	int configInt;
	try{
		configInt = std::stoi(configStr);
	} catch (const std::invalid_argument& e) {
		return 1;
	} catch (const std::out_of_range& e) {
		return 1;
	}
	
	return configInt;
}

void readConfig()
{
	ini::IniFile myIni;
  myIni.load(getDllPath() + modName + ".ini");
	
	fixStandardDamage = (bool)myIni["endless cycles"]["fix standard damage"].as<int>();
	flagCheckNewCycle = myIni["endless cycles"]["event flag check"].as<unsigned int>();
	flagSignNewCycle = myIni["endless cycles"]["event flag sign"].as<unsigned int>();
	flagStartNewCycle = myIni["endless cycles"]["event flag range start"].as<unsigned int>();
}