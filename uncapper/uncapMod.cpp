#include <Windows.h>
#include <algorithm>

#include "ModUtils.h"

#include "uncapMod.h"
#include "config.cpp"
#include "capGetters.cpp"

void PerformPatch(const std::string& aob,
	const std::string& expectedBytes,
	const std::string& newBytes,
	size_t offset)
{
	uintptr_t patchAddress = AobScan(aob);

	if (patchAddress != 0)
	{
		patchAddress += offset;

		ReplaceExpectedBytesAtAddress(patchAddress, expectedBytes, newBytes);
	}
}

DWORD WINAPI MainThread(LPVOID lpParam)
{
	ReadConfig();
	from::DLSY::wait_for_system(-1);
	from::CS::SoloParamRepository::wait_for_params(-1);
	
	Log("Starting uncapper mod");

	std::string buffCapStr = NumberToHexString(stat_caps[0]);
	std::reverse(buffCapStr.begin(),buffCapStr.end());
	for(size_t i = 1 ; i < buffCapStr.size(); i+=2)
    std::swap(buffCapStr[i-1], buffCapStr[i]);
	for(size_t i = 2 ; i < buffCapStr.size()-1; i+=3)
		buffCapStr.insert(i," ");
	
	//uncap stats
	PerformPatch("01 83 a8 02 00 00 ba 63 00 00 00 8b 83 88 02 00 00","63 00 00 00",buffCapStr,7);
	
	//uncap the ??? (I'm not sure what this does)
	PerformPatch("e8 20 8b ec ff 03 45 c7 48 8d 55 a7 83 f8 63 89 45 af 48 8d 4d af 48 0f 4d ca 8b 01","48 0f 4d ca","90 90 90 90",22);
	
	//uncap level-up screen
	{

		uintptr_t comp_loc = reinterpret_cast<uintptr_t>(&compare_levels);
		
		std::string aob = "3b df 7e 2a 83 fb 63 7f 18 48 8b 41 08 48 63 88 68 2a 00 00";
		uintptr_t patchAddress = AobScan(aob)+4;
		
		if (patchAddress > 4)
		{
			return_addresses[0] = patchAddress + 16;
			jump_address = patchAddress + 29;
			
			MemCopy(self_locator(), patchAddress+5, 11);
			
			Hook(patchAddress, comp_loc, 2);
		}
		
	}
	
	//uncap buff preview
	PerformPatch("c7 45 af 63 00 00 00 e8 a5 1c ec ff 03 45 b7 83 f8 63 89 45 a7 48 8d 55 af 48 8d 4d a7 48 0f 4d ca","48 0f 4d ca","90 90 90 90",29);
	PerformPatch("c7 45 a7 63 00 00 00 e8 cb 1c ec ff 03 45 bb 48 8d 55 a7 83 f8 63 89 45 af 48 8d 4d af 48 0f 4d ca","48 0f 4d ca","90 90 90 90",29);
	PerformPatch("c7 45 a7 63 00 00 00 e8 31 1c ec ff 03 45 bf 48 8d 55 a7 83 f8 63 89 45 af 48 8d 4d af 48 0f 4d ca","48 0f 4d ca","90 90 90 90",29);
	PerformPatch("c7 45 a7 63 00 00 00 e8 67 1c ec ff 03 45 c3 48 8d 55 a7 83 f8 63 89 45 af 48 8d 4d af 48 0f 4d ca","48 0f 4d ca","90 90 90 90",29);
	PerformPatch("c7 45 a7 63 00 00 00 e8 2d 1c ec ff 03 45 c7 48 8d 55 a7 83 f8 63 89 45 af 48 8d 4d af 48 0f 4d ca","48 0f 4d ca","90 90 90 90",29);
	PerformPatch("c7 45 a7 63 00 00 00 e8 a3 1b ec ff 03 45 cb 48 8d 55 a7 83 f8 63 89 45 af 48 8d 4d af 48 0f 4d ca","48 0f 4d ca","90 90 90 90",29);
	PerformPatch("c7 45 a7 63 00 00 00 e8 c9 1b ec ff 03 45 cf 48 8d 55 a7 83 f8 63 89 45 af 48 8d 4d af 48 0f 4d ca","48 0f 4d ca","90 90 90 90",29);
	PerformPatch("c7 45 a7 63 00 00 00 e8 6f 1b ec ff 03 45 d3 48 8d 4d af 89 45 af 48 8d 55 a7 83 f8 63 48 0f 4d ca","48 0f 4d ca","90 90 90 90",29);
	PerformPatch("c7 45 a7 63 00 00 00 e8 65 1b ec ff 03 45 d7 48 8d 55 a7 83 f8 63 89 45 af 48 8d 4d af 4d 8b cc 48 0f 4d ca","48 0f 4d ca","90 90 90 90",32);
	
	//hook level up function
	{
		std::string aob = "40 53 48 83 ec 50 8b d9 0f 29 74 24 40 48 8d 4c 24 20 0f 29 7c 24 30 ba c8 00 00 00 c7 44 24 20 ff ff ff ff 48 c7 44 24 28 00 00 00 00 e8 5e d7 6a 00";
		uintptr_t hookAddress = AobScan(aob);
		if (hookAddress != 0){
			uintptr_t func = reinterpret_cast<uintptr_t>(&levelCost);
			uintptr_t funcOrigin = reinterpret_cast<uintptr_t>(&levelCostOriginal);
			
			return_addresses[1] = hookAddress + 18;
			MemCopy(funcOrigin,hookAddress,18);
			Hook(hookAddress, func, 4);
		}
	}
	
	//get last level before level cost is too large to fit.
	int diff = 0;
	int diffOld = 0;
	while (diff >= diffOld){
		diffOld = diff;
		levelMaxGrow++;
		diff = levelCostOriginal(levelMaxGrow+1) - levelCostOriginal(levelMaxGrow);
	}
	Log(levelMaxGrow);
	
	//add scaling above 99
	if (adjustGraph){
		for (auto [id, row] : from::param::CalcCorrectGraph) {
			
			float max3 = row.stageMaxVal3;
			float max4 = row.stageMaxVal4;
			
			float grow3 = row.stageMaxGrowVal3;
			float grow4 = row.stageMaxGrowVal4;
			
			//adjust two handed scaling
			if (max4 == 150.0 && grow4 == 110.0){
				max4 = 99.0;
				grow4 = 100.0;
			}
			//for neatness
			if (max4 == 99.0){
				max4 = 100.0;
			}
			
			float maxStep = max4 - max3;
			float growSize = grow4 - grow3;
			float growStep = growSize / maxStep;
			
			if (grow3 < grow4){
				row.adjPt_maxGrowVal4 = 1.0;
				row.stageMaxVal4 = 999999.0;
				row.stageMaxGrowVal4 = (row.stageMaxVal4 - row.stageMaxVal3) * growStep;
			}
			
		}
	}
	
	return 0;
}

BOOL WINAPI DllMain(HINSTANCE module, DWORD reason, LPVOID)
{
	if (reason == DLL_PROCESS_ATTACH)
	{
		DisableThreadLibraryCalls(module);
		CreateThread(0, 0, &MainThread, 0, 0, NULL);
	}
	return 1;
}