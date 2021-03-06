#include "bytecode_parser.h"
#include "vm/vm_stage.h"
#include <iostream>
#include <chrono>

int main(int argc, char **argv)
{
	if (argc != 3)
	{
		std::cerr << "Expected a single bytecode file as input\n";
		std::exit(1);
	}

	if (strcmp(argv[1], "-i") != 0)
	{
		std::cerr << "Expected -i flag, got " << argv[1] << "\n";
		std::exit(1);
	}

	auto filename = std::string(argv[2]);
	auto executable = fe::vm::parse_bytecode(filename);

	if (executable.byte_length() == 0)
	{
		std::cerr << "Bytecode is empty\n";
		std::exit(1);
	}


	// auto before = std::chrono::high_resolution_clock::now();

	fe::vm::interpret(executable);

	// std::cout << std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::high_resolution_clock::now() - before).count();

	return 0;
}