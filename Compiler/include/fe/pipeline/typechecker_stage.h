#pragma once
#include "fe/data/ext_ast.h"
#include "fe/data/types.h"

namespace fe::ext_ast
{
	types::unique_type typeof_(node& n, ast& ast, type_constraints tc = type_constraints());
	void typecheck(node& n, ast& ast);
}
