#pragma once
#include "fe/data/module.h"
#include "fe/vm/runtime_info.h"

namespace fe::stdlib::io
{
	static module load()
	{
		using namespace types;
		auto ui64t = ui64();
		auto voidtt = voidt();
		return module_builder()
		  .set_name({ "std", "io" })
		  .add_native_function(
		    vm::PRINT, "print",
		    make_unique(function_type(make_unique(ui64t), make_unique(voidtt))))
		  .add_native_function(
		    vm::PRINTLN, "println",
		    make_unique(function_type(make_unique(ui64t), make_unique(voidtt))))
		  .build();
	}
} // namespace fe::stdlib::io