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

bool returnTrue(char8_t flag){
    return true;
}

const wchar_t * getMessage(uintptr_t messageRepository, uint32_t _1,uint32_t msgBnd, uint32_t msgId){
	if (msgId == 103090 && msgBnd == 200){
		return L"Talisman 5";
	}
	
	return getMessageOriginal(messageRepository,_1,msgBnd,msgId);
}

void menuTypeHook(HookContext* context){
    uintptr_t* data = (uintptr_t*)((context->r8)+0x38);
    char8_t &menuType = *(char8_t*)(*data + 0xC);
    if (menuType == 21){
        menuType = 20;
    }
}

DWORD WINAPI MainThread(LPVOID lpParam){
    from::DLSY::wait_for_system(-1);
    MH_STATUS status = MH_Initialize();
    if (!CallHook::initialize()) return 0;
	Log("MinHook Status: ",MH_StatusToString(status));

    {
        uintptr_t address = AobScan(enableSlotAob);
        if (address != 0){
            uintptr_t* func = (uintptr_t*)(getAddressFromMemory(getAddressFromMemory(address,enableSlotOffset,4),enableSlotOffset2,4) + 24*8 );
            *func = (uintptr_t)&returnTrue;
        }
    }
    {
        uintptr_t address = AobScan(menuTypeAob);
        if (address != 0){
            auto hook1 = new CallHookTemplate<ContextHook>(reinterpret_cast<void *>(address + menuTypeOffset), menuTypeHook);
        }
    }
    {
        uintptr_t address = AobScan(slotMaxAob);
        if (address != 0){
            uint32_t* slotMax = (uint32_t*)(address + slotMaxOffset);
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