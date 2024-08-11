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

DWORD WINAPI MainThread(LPVOID lpParam){
    from::DLSY::wait_for_system(-1);
    performPatch(enableSlotAob,"74 11","90 90",enableSlotOffset);
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