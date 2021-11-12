// CellularAutomata.cpp : Defines the entry point for the application.


#include "CellularAutomata1D.h"
#include <list>
#include <vector>
#include <tuple>


std::string* product(std::string iterable, int repeat)
{	//TODO try to port https://github.com/tdworowy/cellular_automata_the_game/blob/master/scripts/utils.gd
	int resultSize = pow(iterable.length(), repeat);
	std::list<std::string> pools;
	
	std::string* result = new std::string[resultSize]; 
	for (int i = 0; i < repeat; i++) {
		result[i] = "0";
	}// memory alocation

		
	for(int i = 0; i < repeat; i++) {
		pools.push_back(iterable);
	}
	for (std::list<std::string>::iterator pool = pools.begin(); pool != pools.end(); ++pool) { // mayby recursive solution will be better
		for (int i = 0; i < resultSize; i++) {
			for (int j = 0; j < pool->length(); j++) {
				const int tempArrSize = 1;
				char* tempArr[tempArrSize] = { &pool->at(j) };
				std::vector<char> temp;
				
				temp.reserve(sizeof(result[i])/ sizeof(result[i]) + tempArrSize);
				temp.insert(temp.end(), std::begin(result[i]), std::end(result[i]));
				temp.insert(temp.end(), std::begin(tempArr), std::end(tempArr));

				std::string s(temp.begin(), temp.end());
				std::cout << s;
				// TODO make it works https://stackoverflow.com/questions/12791266/c-concatenate-two-int-arrays-into-one-larger-array/12791344
				// https://stackoverflow.com/questions/62135867/c-const-char-with-begin-and-end

			}
		}
	}

	return result;
}
void deleteArray(int resultSize, int** arr)
{
	for (int i = 0; i < resultSize; i++)
		delete[] arr[i];
	delete[] arr;
}
std::string nNary(int number, int n)
{	
	if(number == 0)
	{
	   return "0";
	}
	std::string nums;
	while(number != 0)
	{
		div_t result = std::div(number, n);
		number = result.quot;
		int r = result.rem;
		nums += std::to_string(r);
	}
	std::reverse(std::begin(nums),std::end(nums));
	return nums;

}
std::string WolframNumberToBin(int worframNumber, int PossibleStates, int ColoursCount)
{
	std::string initWolframNumber = nNary(worframNumber, ColoursCount);
	int temp = PossibleStates - initWolframNumber.length();
	std::string wolframNumber = "";
	for (int i = 0; i < temp; i++)
	{
		wolframNumber += "0";
	}
	wolframNumber += initWolframNumber;
	std::reverse(std::begin(wolframNumber), std::end(wolframNumber));
	return wolframNumber;
}
/*std::list<RuleSegment> generateRule(int initWorframNumber, int neighborhoodSize, std::string colours)
{
	int colcoloursCount = colours.length();
	long int possibleStates = pow(colcoloursCount,neighborhoodSize);
	std::string worframNumber = WolframNumberToBin(initWorframNumber, possibleStates, colcoloursCount);
	std::list<RuleSegment> rule;
	
	std::list<std::tuple<int, int>> combinations = product(colours, neighborhoodSize);
	int index = 0;
	for (std::list<std::tuple<int, int>>::iterator it = combinations.begin(); it != combinations.end(); ++it)
	{
		rule.push_back(RuleSegment{ *it, (int)worframNumber[index] });
		index++;
	}
	return rule;

}  */

int main()
{
	product("123",3);
	return 0;
}

