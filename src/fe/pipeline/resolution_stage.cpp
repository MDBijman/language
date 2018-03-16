#include "fe/data/extended_ast.h"
#include "fe/data/scope_environment.h"
#include <locale>

namespace fe::extended_ast
{
	void integer::resolve(scope_environment& s_env) {}

	void string::resolve(scope_environment& s_env) {}

	void identifier::resolve(scope_environment& s_env)
	{
		auto access_pattern = s_env.resolve_reference(*this).value();
		this->scope_distance = access_pattern.first;
		this->offsets = access_pattern.second;
	}

	void tuple::resolve(scope_environment& s_env)
	{
		for (decltype(auto) elem : this->children) elem->resolve(s_env);
	}

	void function_call::resolve(scope_environment& s_env)
	{
		if(auto access_pattern = s_env.resolve_reference(this->id); access_pattern.has_value())
		{
			this->id.scope_distance = access_pattern.value().first;
			this->id.offsets = access_pattern.value().second;
		}
		else if (auto access_pattern = s_env.resolve_type(this->id); access_pattern.has_value())
		{
			this->id.scope_distance = access_pattern.value();
		}
		else
		{
			throw resolution_error{ "Cannot resolve function call name" };
		}

		this->params->resolve(s_env);
	}

	void match_branch::resolve(scope_environment& s_env)
	{
		s_env.push();
		this->test_path->resolve(s_env);
		this->code_path->resolve(s_env);
		s_env.pop();
	}

	void match::resolve(scope_environment& s_env)
	{
		this->expression->resolve(s_env);
		for (decltype(auto) branch : this->branches) branch.resolve(s_env);
	}

	void block::resolve(scope_environment& s_env)
	{
		s_env.push();
		for (decltype(auto) child : this->children) child->resolve(s_env);
		s_env.pop();
	}

	void module_declaration::resolve(scope_environment& s_env) {}

	void atom_declaration::resolve(scope_environment& s_env){}

	void tuple_declaration::resolve(scope_environment& s_env)
	{
		for (decltype(auto) elem : this->elements) elem->resolve(s_env);
	}

	void function::resolve(scope_environment& s_env)
	{
		s_env.declare(this->name, extended_ast::identifier({ "_function" }));
		s_env.define(this->name);

		s_env.push();
		this->from->resolve(s_env);

		// Define parameters
		std::function<void(node*, scope_environment& s_env)> define = [&](node* n, scope_environment& s_env) -> void {
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
					s_env.declare(atom_dec->name, *dynamic_cast<identifier*>(type_expression_name->type.get()));
					s_env.define(atom_dec->name);
				}
				else
				{
					throw resolution_error{ "Type expression name resolution not supported yet" };
				}
			}
		};
		define(this->from.get(), s_env);

		this->to->resolve(s_env);
		this->body->resolve(s_env);
		s_env.pop();
	}

	void type_definition::resolve(scope_environment& s_env)
	{
		s_env.define_type(this->id, this->types);
	}

	void export_stmt::resolve(scope_environment& s_env)
	{
		for (auto& child : this->names)
		{
			s_env.resolve_type(child);
		}
	}

	void identifier_tuple::resolve(scope_environment& s_env) {}

	void assignment::resolve(scope_environment& s_env)
	{
		if (std::holds_alternative<extended_ast::identifier>(this->lhs))
		{
			auto& lhs_id = std::get<extended_ast::identifier>(this->lhs);
			s_env.declare(lhs_id, this->type_name);
			this->value->resolve(s_env);
			s_env.define(lhs_id);
		}
		else if (std::holds_alternative<extended_ast::identifier_tuple>(this->lhs))
		{
			throw resolution_error{ "Identifier tuples not supported yet in name resolution" };
		}
	}

	void type_tuple::resolve(scope_environment& s_env) {}
	void type_atom::resolve(scope_environment& s_env) {}
	void function_type::resolve(scope_environment& s_env) {}
	void reference_type::resolve(scope_environment& s_env) {}
	void array_type::resolve(scope_environment& s_env) {}

	void reference::resolve(scope_environment& s_env)
	{
		this->child->resolve(s_env);
	}

	void array_value::resolve(scope_environment& s_env)
	{
		for (auto& child : this->children)
			child->resolve(s_env);
	}

	void equality::resolve(scope_environment& s_env)
	{
		this->left->resolve(s_env);
		this->right->resolve(s_env);
	}

	void addition::resolve(scope_environment& s_env)
	{
		this->left->resolve(s_env);
		this->right->resolve(s_env);
	}

	void subtraction::resolve(scope_environment& s_env)
	{
		this->left->resolve(s_env);
		this->right->resolve(s_env);
	}

	void multiplication::resolve(scope_environment& s_env)
	{
		this->left->resolve(s_env);
		this->right->resolve(s_env);
	}

	void division::resolve(scope_environment& s_env)
	{
		this->left->resolve(s_env);
		this->right->resolve(s_env);
	}

	void array_index::resolve(scope_environment& s_env)
	{
		this->array_exp->resolve(s_env);
		this->index_exp->resolve(s_env);
	}

	void while_loop::resolve(scope_environment& s_env)
	{
		this->test->resolve(s_env);
		this->body->resolve(s_env);
	}

	void import_declaration::resolve(scope_environment& s_env) {}
}