// CellularAutomata.cpp : Defines the entry point for the application.


#include "CellularAutomata1D.h"
#include <list>
#include <tuple>



std::list<std::tuple<int, int>> product(std::string iterables, int repear)
{	//TODO
	return std::list<std::tuple<int, int>>();
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
std::list<RuleSegment> generateRule(int initWorframNumber, int neighborhoodSize, std::string colours)
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

}

int main()
{
	std::cout << WolframNumberToBin(110, 8, 2);
	return 0;
}

