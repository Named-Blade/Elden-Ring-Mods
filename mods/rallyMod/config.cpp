#include <inicpp.h>
#include "rallyMod.hpp"

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

void readConfig()
{
	ini::IniFile myIni;
	std::string path = getDllPath() + modName + ".ini";
  myIni.load(getDllPath() + modName + ".ini");
	
	rallyConfigTime = myIni["rally mod"]["default rally timer"].as<float>();
	rallyConfigDecay = myIni["rally mod"]["rally decay"].as<float>();
	rallyResetOnHit = (bool)myIni["rally mod"]["reset rally time on hit"].as<int>();
	rallyKeepOnHeal = (bool)myIni["rally mod"]["reset rally time on heal"].as<int>();
	rallyResetOnHeal = (bool)myIni["rally mod"]["keep rally on heal"].as<int>();
}