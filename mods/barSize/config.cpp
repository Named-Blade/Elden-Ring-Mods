#include "ini.h"
#include "barSize.hpp"

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

void ReadConfig()
{
	INIFile config(getDllPath() + modName + ".ini");
	INIStructure ini;
	
	if (config.read(ini))
	{
		isLinearFirst = strToBool(ini["bar size mod"].get("use linear size first"));
		maxDisplaySize = strToFloat(ini["bar size mod"].get("max display size"));
		squishDisplay = strToBool(ini["bar size mod"].get("display size squish"));
	}
}