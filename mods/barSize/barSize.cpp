#include "barSize.hpp"
#include "DLLMain.hpp"

auto playerMaxHealth = PointerChain::make<int, true>(worldCharManPtr,0x0,localPlayerOffset,playerInstanceOffset,playerModulesOffset,playerDataOffset,maxHealthDDOffset);
auto playerMaxStamina = PointerChain::make<int, true>(worldCharManPtr,0x0,localPlayerOffset,playerInstanceOffset,playerModulesOffset,playerDataOffset,maxStaminaOffset);
auto playerMaxFocus = PointerChain::make<int, true>(worldCharManPtr,0x0,localPlayerOffset,playerInstanceOffset,playerModulesOffset,playerDataOffset,maxFocusOffset);

struct set_bar_size_task: public from::CS::CSEzTask {
	void eztask_execute(from::FD4::FD4TaskData* data) override {
		for (auto [id, row] : from::param::MenuCommonParam) {
			
			int maxHealth;
			if (playerMaxHealth != nullptr){
				maxHealth = std::round(*playerMaxHealth/healthConfig.maxDisplaySize);
			} else {
				maxHealth = maxHealthParam;
			}
			if (healthConfig.isLinearFirst && maxHealth < maxHealthParam){
				row.playerMaxHpLimit = maxHealthParam;
			} else {
				row.playerMaxHpLimit = maxHealth;
			}
			
			int maxFocus;
			if (playerMaxFocus != nullptr){
				maxFocus = std::round(*playerMaxFocus/focusConfig.maxDisplaySize);
			} else {
				maxFocus = maxFocusParam;
			}
			if (focusConfig.isLinearFirst && maxFocus < maxFocusParam){
				row.playerMaxMpLimit = maxFocusParam;
			} else {
				row.playerMaxMpLimit = maxFocus;
			}
			
			int maxStamina;
			if (playerMaxStamina != nullptr){
				maxStamina = std::round(*playerMaxStamina/staminaConfig.maxDisplaySize);
			} else {
				maxStamina = maxStaminaParam;
			}
			if (staminaConfig.isLinearFirst && maxStamina < maxStaminaParam){
				row.playerMaxSpLimit = maxStaminaParam;
			} else {
				row.playerMaxSpLimit = maxStamina;
			}
			
		}
	}
};

void base() {
	std::string healthSection = "Health";
	std::string focusSection = "Focus";
	std::string staminaSection = "Stamina";
	readConfig(
		std::forward_as_tuple(healthConfig.isLinearFirst,healthSection,"use linear size first"_s),
		std::forward_as_tuple(healthConfig.maxDisplaySize,healthSection,"max display size"_s),
		std::forward_as_tuple(healthConfig.squishDisplay,healthSection,"display size squish"_s),
		std::forward_as_tuple(focusConfig.isLinearFirst,focusSection,"use linear size first"_s),
		std::forward_as_tuple(focusConfig.maxDisplaySize,focusSection,"max display size"_s),
		std::forward_as_tuple(focusConfig.squishDisplay,focusSection,"display size squish"_s),
		std::forward_as_tuple(staminaConfig.isLinearFirst,staminaSection,"use linear size first"_s),
		std::forward_as_tuple(staminaConfig.maxDisplaySize,staminaSection,"max display size"_s),
		std::forward_as_tuple(staminaConfig.squishDisplay,staminaSection,"display size squish"_s)
	);
	
	from::DLSY::wait_for_system(-1);
	from::CS::SoloParamRepository::wait_for_params(-1);
	
	for (auto [id, row] : from::param::MenuCommonParam) {
		{
			float displaySize;
			if (healthConfig.squishDisplay){
				displaySize = 1.0/healthConfig.maxDisplaySize;
			} else {
				displaySize = 1.0;
			}
			maxHealthParam = (int)row.playerMaxHpLimit*displaySize;
		}
		{
			float displaySize;
			if (focusConfig.squishDisplay){
				displaySize = 1.0/focusConfig.maxDisplaySize;
			} else {
				displaySize = 1.0;
			}
			maxFocusParam = (int)row.playerMaxMpLimit*displaySize;
		}
		{
			float displaySize;
			if (staminaConfig.squishDisplay){
				displaySize = 1.0/staminaConfig.maxDisplaySize;
			} else {
				displaySize = 1.0;
			}
			maxStaminaParam = (int)row.playerMaxSpLimit*displaySize;
		}
	}
	
	from::unique_ptr<set_bar_size_task> bar_size_task =
		from::make_unique<set_bar_size_task>();
	bar_size_task->register_task(from::CS::CSTaskGroup::ScaleformStep);
	
	Sleep(INFINITE);
}
