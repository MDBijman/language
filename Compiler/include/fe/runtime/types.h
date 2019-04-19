#pragma once
#include "fe/data/types.h"
#include "fe/data/module.h"
#include "fe/data/bytecode.h"

namespace fe::stdlib::typedefs
{
static module load()
{
	using namespace types;
	auto i8t = i8();
	auto ui8t = ui8();
	auto i16t = i16();
	auto ui16t = ui16();
	auto i32t = i32();
	auto ui32t = ui32();
	auto i64t = i64();
	auto ui64t = ui64();
	auto strt = str();
	auto boolt = types::boolean();
	
	return module_builder()
		.set_name({"std"})
		.add_type("i8", make_unique(i8t))
		.add_type("ui8", make_unique(ui8t))
		.add_type("i16", make_unique(i16t))
		.add_type("ui16", make_unique(ui16t))
		.add_type("i32", make_unique(i32t))
		.add_type("ui32", make_unique(ui32t))
		.add_type("i64", make_unique(i64t))
		.add_type("ui64", make_unique(ui64t))
		.add_type("str", make_unique(strt))
		.add_type("bool", make_unique(boolt))
		.build();
}
} // namespace fe::stdlib::typedefs