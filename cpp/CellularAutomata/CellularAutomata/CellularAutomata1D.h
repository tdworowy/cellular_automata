﻿// CellularAutomata.h : Include file for standard system include files,
// or project specific include files.

#pragma once

#include <iostream>
#include <string>
#include <tuple>
#include <list>


struct RuleSegment {
	std::tuple<int, int> neighborhood;
	int type;
};

std::string nNary(int number, int n);
std::string WolframNumberToBin(int worframNumber, int PossibleStates, int ColoursCount);
std::list<RuleSegment> generateRule(int worframNumber, int neighborhoodSize, std::string colours);
std::list<std::tuple<int, int>> product(std::string iterables, int repear);