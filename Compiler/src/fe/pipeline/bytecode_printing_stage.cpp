#include "fe/pipeline/bytecode_printing_stage.h"

#include <experimental/filesystem>
#include <iostream>
#include <string>
#include <fstream>
namespace fe::vm
{
	void print_bytecode(const std::string& path, executable& e)
	{
		std::experimental::filesystem::create_directory("out");
		std::experimental::filesystem::path p("./out/" + path + ".bc");
		std::ofstream f;
		f.open(p, std::ofstream::out | std::ofstream::trunc);
		f << e.to_string();
		f.close();
	}
}
