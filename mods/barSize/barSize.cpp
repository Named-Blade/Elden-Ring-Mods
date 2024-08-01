#include "barSize.hpp"
#include "DLLMain.hpp"

template <typename T>
T ptrChain (std::vector<uintptr_t> ptrs, T defaultArg){
	if (ptrs.empty()) {
		return defaultArg;
	}
	uintptr_t current = ptrs[0];
	for (size_t i = 1; i < ptrs.size()-1; ++i){
		if (current == 0){
			return defaultArg;
		}
		current = *reinterpret_cast<uintptr_t*>(current + ptrs[i]);
	}
	if (current == 0){
		return defaultArg;
	}
	T result = *reinterpret_cast<T*>(current + ptrs.back());
	return result; 
}

int getMaxHealth()
{
	std::vector<uintptr_t> ptrs = {worldCharManPtr,0x0,localPlayerOffset,playerInstanceOffset,playerModulesOffset,playerDataOffset,maxHealthDDOffset};
	return ptrChain<int>(ptrs,maxHealthParam);
}
int getMaxStamina()
{
	std::vector<uintptr_t> ptrs = {worldCharManPtr,0x0,localPlayerOffset,playerInstanceOffset,playerModulesOffset,playerDataOffset,maxStaminaOffset};
	return ptrChain<int>(ptrs,maxStaminaParam);
}
int getMaxFocus()
{
	std::vector<uintptr_t> ptrs = {worldCharManPtr,0x0,localPlayerOffset,playerInstanceOffset,playerModulesOffset,playerDataOffset,maxFocusOffset};
	return ptrChain<int>(ptrs,maxFocusParam);
}

struct set_bar_size_task: public from::CS::CSEzTask {
	void eztask_execute(from::FD4::FD4TaskData* data) override {
		for (auto [id, row] : from::param::MenuCommonParam) {
			
			int maxHealth = (int)getMaxHealth()/maxDisplaySize;
			if (isLinearFirst && maxHealth < maxHealthParam){
				row.playerMaxHpLimit = maxHealthParam;
			} else {
				row.playerMaxHpLimit = maxHealth;
			}
			
			int maxStamina = (int)getMaxStamina()/maxDisplaySize;
			if (isLinearFirst && maxStamina < maxStaminaParam){
				row.playerMaxSpLimit = maxStaminaParam;
			} else {
				row.playerMaxSpLimit = maxStamina;
			}
			
			int maxFocus = (int)getMaxFocus()/maxDisplaySize;
			if (isLinearFirst && maxFocus < maxFocusParam){
				row.playerMaxMpLimit = maxFocusParam;
			} else {
				row.playerMaxMpLimit = maxFocus;
			}
			
		}
	}
};

void base() {
	std::string section = "bar size mod";
	readConfig(
		std::forward_as_tuple(isLinearFirst,section,"use linear size first"_s),
		std::forward_as_tuple(maxDisplaySize,section,"max display size"_s),
		std::forward_as_tuple(squishDisplay,section,"display size squish"_s)
	);
	
	from::DLSY::wait_for_system(-1);
	from::CS::SoloParamRepository::wait_for_params(-1);
	
	for (auto [id, row] : from::param::MenuCommonParam) {
		float displaySize;
		if (squishDisplay){
			displaySize = 1.0/maxDisplaySize;
		} else {
			displaySize = 1.0;
		}
		maxHealthParam = (int)row.playerMaxHpLimit*displaySize;
		maxFocusParam = (int)row.playerMaxMpLimit*displaySize;
		maxStaminaParam = (int)row.playerMaxSpLimit*displaySize;
	}
	
	from::unique_ptr<set_bar_size_task> bar_size_task =
		from::make_unique<set_bar_size_task>();
	bar_size_task->register_task(from::CS::CSTaskGroup::ScaleformStep);
	
	Sleep(INFINITE);
}
