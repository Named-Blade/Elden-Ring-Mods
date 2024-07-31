#include "ini.h"
#include "damageMod.hpp"

using namespace mINI;

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

float strToInt (std::string configStr)
{
	float configInt;
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
	INIFile config(getDllPath() + modName + ".ini");
	INIStructure ini;
	
	if (config.read(ini))
	{
		healAbsorb = strToBool(ini["damage mod"].get("healing absorption"));
		flatterDefenses = strToBool(ini["damage mod"].get("flatter defenses"));
		chipDamage = (float)strToInt(ini["damage mod"].get("chip damage"));
		splitDamageFix = strToBool(ini["damage mod"].get("split damage fix"));
		heavenOrHell = strToBool(ini["damage mod"].get("Heaven or Hell"));
	}
}