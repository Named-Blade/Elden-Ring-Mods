#include "talismanSlot.hpp"

void performPatch(const std::string& aob,
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

const wchar_t * getMessage(uintptr_t messageRepository, uint32_t _1,uint32_t msgBnd, uint32_t msgId){
	if (msgId == 103090 && msgBnd == 200){
		return L"Talisman 5";
	}
	
	return getMessageOriginal(messageRepository,_1,msgBnd,msgId);
}

DWORD WINAPI MainThread(LPVOID lpParam){
    from::DLSY::wait_for_system(-1);
    MH_STATUS status = MH_Initialize();
	Log("MinHook Status: ",MH_StatusToString(status));

    performPatch(enableSlotAob,"74 11","90 90",enableSlotOffset);
    {
        uintptr_t address = AobScan(slotMaxAob);
        if (address != 0){
            uint32_t* slotMax = (uint32_t*)(address + slotMaxOffset);
            Log(*slotMax);
            *slotMax = 5;
        }
    }

    hookCall(getMessage,getMessageAob,getMessageOffset,4,&getMessageOriginal);

    MH_STATUS applyStatus = MH_ApplyQueued();
	Log("Apply Status: ",MH_StatusToString(applyStatus));
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