#include <cmath>

//this is an approximation
int ds2LevelCost(int level){
	if (level < 1) {
		return ds2LevelCost(1);
	} else if (level < 200){
		return round(464.3580251092935 + (30.4847065*pow(level,1)) + (0.834325289*pow(level,2)) + (0.00190073572*pow(level,3)));
	} else if (level < 250) {
		return round(-6823970.050517185 + (101635.938*pow(level,1)) + (-510.821554*pow(level,2)) + (0.872787992*pow(level,3)));
	} else {
		return round(102089.72217704158 + (851.019699*pow(level,1)) + (-0.735832315*pow(level,2)) + (0.00168066462*pow(level,3)));
	}
}