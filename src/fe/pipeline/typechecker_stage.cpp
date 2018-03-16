#pragma once
#include "fe/data/extended_ast.h"
#include "fe/data/type_environment.h"

namespace fe::extended_ast
{
	void integer::typecheck(type_environment& t_env)
	{
		this->set_type(new types::atom_type("std.i32"));
	}

	void string::typecheck(type_environment& t_env)
	{
		this->set_type(new types::atom_type("std.str"));
	}

#pragma optimize("", off)
	void identifier::typecheck(type_environment& env)
	{
		auto t = env.typeof(*this);
		if(!t.has_value())
			throw typecheck_error{ "Type environment error" };

		set_type(t.value().get().copy());
	}

	void tuple::typecheck(type_environment& env)
	{
		auto new_type = types::product_type();

		for (decltype(auto) element : children)
		{
			element->typecheck(env);

			new_type.product.push_back(types::unique_type(element->get_type().copy()));
		}

		set_type(new_type.copy());
	}

#pragma optimize("", off)
	void function_call::typecheck(type_environment& env)
	{
		params->typecheck(env);

		auto& argument_type = params->get_type();

		auto type = env.typeof(this->id);
		if (!type.has_value())
			throw typecheck_error{ "Function name cannot be resolved" };

		auto& function_or_type = type.value().get();
		if (auto function_type = dynamic_cast<types::function_type*>(&function_or_type))
		{
			// Check function signature against call signature
			if (!(argument_type == function_type->from.get()))
			{
				throw typecheck_error{
					"Function call from signature does not match function signature:\n"
					+ argument_type.to_string() + "\n"
					+ function_type->from->to_string()
				};
			}

			set_type(types::unique_type(function_type->to->copy()));
		}
		else if (auto product_type = dynamic_cast<types::product_type*>(&function_or_type))
		{
			// Check type signature against call signature
			if (!(argument_type == product_type))
			{
				throw typecheck_error{
					"Function call to signature does not match function signature:\n"
					+ argument_type.to_string() + "\n"
					+ product_type->to_string()
				};
			}

			set_type(types::unique_type(product_type->copy()));
		}
		else
		{
			throw typecheck_error{
				"Function call can only call constructor or function"
			};
		}
	}

	void match_branch::typecheck(type_environment& env)
	{
		env.push();
		test_path->typecheck(env);
		code_path->typecheck(env);

		// Check the validity of the type of the test path
		auto& test_type = test_path->get_type();
		if (!(types::atom_type("boolean") == &test_type))
			throw typecheck_error{ std::string("Branch number does not have a boolean test") };

		set_type(types::unique_type(code_path->get_type().copy()));
		env.pop();
	}

	void match::typecheck(type_environment& env)
	{
		types::type* common_type = new types::unset_type();

		for (uint32_t branch_count = 0; branch_count < branches.size(); branch_count++)
		{
			branches.at(branch_count).typecheck(env);

			// In first iteration
			if (types::unset_type() == common_type)
			{
				common_type = branches.at(branch_count).get_type().copy();
				continue;
			}

			// Check other elements are the same
			if (!(branches.at(branch_count).get_type() == common_type))
				throw typecheck_error{ std::string("Branch is of a different type than those before it") };
		}

		set_type(common_type);
	}

	void block::typecheck(type_environment& env)
	{
		env.push();
		types::unique_type final_type = nullptr;
		for (decltype(auto) element : children)
		{
			element->typecheck(env);
			final_type.reset(element->get_type().copy());
		}

		if (final_type != nullptr)
			set_type(final_type->copy());
		else
			set_type(new types::unset_type());
		env.pop();
	}

	void module_declaration::typecheck(type_environment& env)
	{
	}

	void atom_declaration::typecheck(type_environment& env)
	{
		type_expression->typecheck(env);
		env.set_type(name, types::unique_type(type_expression->get_type().copy()));
		set_type(type_expression->get_type().copy());
	}

	void tuple_declaration::typecheck(type_environment& env)
	{
		types::product_type res;
		for (decltype(auto) elem : elements)
		{
			elem->typecheck(env);

			if (const auto atom = dynamic_cast<atom_declaration*>(elem.get()))
			{
				res.product.push_back(types::unique_type(atom->get_type().copy()));
			}
		}
		set_type(res.copy());
	}

	void function::typecheck(type_environment& env)
	{
		env.push();

		from->typecheck(env);
		to->typecheck(env);

		auto this_type = types::function_type(
			types::unique_type(from->get_type().copy()),
			types::unique_type(to->get_type().copy())
		);

		this->set_type(types::make_unique(this_type));
		env.set_type(name, types::make_unique(this_type), 1);

		// Define parameters
		std::function<void(node*, type_environment& s_env)> define = [&](node* n, type_environment& s_env) -> void {
			if (auto tuple_dec = dynamic_cast<tuple_declaration*>(n))
			{
				for (decltype(auto) child : tuple_dec->elements)
				{
					define(child.get(), s_env);
				}
			}
			else if (auto atom_dec = dynamic_cast<atom_declaration*>(n))
			{
				if (auto type_expression_name = dynamic_cast<type_atom*>(atom_dec->type_expression.get()))
				{
					auto resolved_type =
						s_env.resolve_type(*dynamic_cast<identifier*>(type_expression_name->type.get()));
					if (!resolved_type.has_value()) throw resolution_error{ "Paremeter type unknown" };

					s_env.set_type(atom_dec->name, types::unique_type(resolved_type.value().get().copy()));
				}
				else
				{
					throw resolution_error{ "Type expression name resolution not supported yet" };
				}
			}
		};
		body->typecheck(env);

		if (!(body->get_type() == &to->get_type()))
		{
			throw typecheck_error{ "Given return type is not the same as the type of the body" };
		}
		env.pop();
	}

	void type_definition::typecheck(type_environment& env)
	{
		types->typecheck(env);
		env.define_type(id, types::unique_type(types->get_type().copy()));
		env.set_type(id, types::unique_type(new types::function_type(
			types::unique_type(types->get_type().copy()),
			types::unique_type(types->get_type().copy())
		)));
		id.set_type(types::unique_type(types->get_type().copy()));
		set_type(types::unique_type(types->get_type().copy()));
	}

	void export_stmt::typecheck(type_environment& env)
	{
		set_type(new types::atom_type("void"));
	}

	void assignment::typecheck(type_environment& env)
	{
		value->typecheck(env);

		std::function<void(std::variant<identifier, identifier_tuple>&, types::type&)> typecheck_tuple = [&](std::variant<identifier, identifier_tuple>& lhs, types::type& type) {
			if (std::holds_alternative<identifier_tuple>(lhs))
			{
				auto product_type = dynamic_cast<types::product_type*>(&type);
				assert(product_type != nullptr);

				auto& ids = std::get<identifier_tuple>(lhs);
				assert(ids.content.size() > 1);
				assert(ids.content.size() == product_type->product.size());

				for (auto i = 0; i < ids.content.size(); i++)
				{
					typecheck_tuple(ids.content.at(i), *product_type->product.at(i));
				}
			}
			else if (std::holds_alternative<identifier>(lhs))
			{
				auto& id = std::get<identifier>(lhs);

				env.define_type(id, types::unique_type(type.copy()));
			}
			else assert(!"Invalid variant contents");
		};
		typecheck_tuple(this->lhs, value->get_type());

		set_type(new types::atom_type("void"));
	}

	void type_tuple::typecheck(type_environment& env)
	{
		types::product_type result;

		for (auto& child : elements)
		{
			child->typecheck(env);
			result.product.push_back(types::unique_type(child->get_type().copy()));
		}

		set_type(result.copy());
	}

	void type_atom::typecheck(type_environment& env)
	{
		auto type = env.resolve_type(*dynamic_cast<identifier*>(this->type.get()));
		if (!type.has_value())
			throw typecheck_error{ "Unknown type" };

		set_type(type.value().get().copy());
	}

	void function_type::typecheck(type_environment& env)
	{
		from->typecheck(env);
		to->typecheck(env);

		set_type(new types::function_type(from->get_type(), to->get_type()));
	}

	void reference_type::typecheck(type_environment & env)
	{
		child->typecheck(env);
		set_type(new types::reference_type(types::make_unique(child->get_type())));
	}

	void array_type::typecheck(type_environment& env)
	{
		child->typecheck(env);

		set_type(types::make_unique(types::array_type(child->get_type())));
	}

	void reference::typecheck(type_environment & env)
	{
		child->typecheck(env);

		set_type(types::make_unique(types::reference_type(child->get_type())));
	}

	void array_value::typecheck(type_environment& env)
	{
		for (decltype(auto) child : children)
			child->typecheck(env);

		if (children.size() > 0)
		{
			const auto element_type = children.at(0)->get_type().copy();

			for (decltype(auto) child : children)
			{
				if (!(child->get_type() == element_type))
					throw typecheck_error{ "All types in an array must be equal" };
			}

			set_type(types::make_unique(types::array_type(types::unique_type(element_type))));
		}
		else
			set_type(types::make_unique(types::array_type(types::atom_type{ "void" })));
	}

	void equality::typecheck(type_environment& env)
	{
		left->typecheck(env);
		right->typecheck(env);

		if (!(types::atom_type{ "std.i32" } == &left->get_type()))
		{
			throw typecheck_error{ "Left side of equality must be a number" };
		}

		if (!(types::atom_type{ "std.i32" } == &right->get_type()))
		{
			throw typecheck_error{ "Right side of equality must be a number" };
		}

		set_type(types::make_unique(types::atom_type{ "boolean" }));
	}

	void addition::typecheck(type_environment& env)
	{
		left->typecheck(env);
		right->typecheck(env);

		if (!(types::atom_type{ "std.i32" } == &left->get_type()))
		{
			throw typecheck_error{ "Left side of addition must be a number" };
		}

		if (!(types::atom_type{ "std.i32" } == &right->get_type()))
		{
			throw typecheck_error{ "Right side of addition must be a number" };
		}

		set_type(types::make_unique(types::atom_type{ "std.i32" }));
	}

	void subtraction::typecheck(type_environment& env)
	{
		left->typecheck(env);
		right->typecheck(env);

		if (!(left->get_type() == &types::atom_type{ "std.i32" }))
		{
			throw typecheck_error{ "Left side of subtraction must be a number" };
		}

		if (!(right->get_type() == &types::atom_type{ "std.i32" }))
		{
			throw typecheck_error{ "Right side of subtraction must be a number" };
		}

		set_type(types::make_unique(types::atom_type{ "std.i32" }));
	}

	void multiplication::typecheck(type_environment& env)
	{
		left->typecheck(env);
		right->typecheck(env);

		if (!(left->get_type() == &types::atom_type{ "std.i32" }))
		{
			throw typecheck_error{ "Left side of multiplication must be a number" };
		}

		if (!(right->get_type() == &types::atom_type{ "std.i32" }))
		{
			throw typecheck_error{ "Right side of multiplication must be a number" };
		}

		set_type(types::make_unique(types::atom_type{ "std.i32" }));
	}

	void division::typecheck(type_environment& env)
	{
		left->typecheck(env);
		right->typecheck(env);

		if (!(left->get_type() == &types::atom_type{ "std.i32" }))
		{
			throw typecheck_error{ "Left side of division must be a number" };
		}

		if (!(right->get_type() == &types::atom_type{ "std.i32" }))
		{
			throw typecheck_error{ "Right side of division must be a number" };
		}

		set_type(types::make_unique(types::atom_type{ "std.i32" }));
	}

	void array_index::typecheck(type_environment& env)
	{
		array_exp->typecheck(env);
		index_exp->typecheck(env);

		if (const auto type = dynamic_cast<types::array_type*>(&array_exp->get_type()))
		{
			set_type(type->element_type->copy());
		}
		else
		{
			throw typecheck_error{ "Array expression must be of type array" };
		}

		if (!(index_exp->get_type() == &types::atom_type{ "std.i32" }))
		{
			throw typecheck_error{ "Array index must be an integer" };
		}
	}

	void while_loop::typecheck(type_environment& env)
	{
		test->typecheck(env);
		body->typecheck(env);
		set_type(new types::unset_type());

		if (!(types::atom_type("boolean") == &test->get_type()))
		{
			throw typecheck_error{ "Test branch of while loop must have boolean type" };
		}
	}

	void import_declaration::typecheck(type_environment& env) {}
}