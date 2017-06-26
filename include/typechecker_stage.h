#pragma once
#include <memory>

#include "core_ast.h"
#include "extended_ast.h"

namespace fe
{
	struct type_environment
	{
		type_environment() {}
		type_environment(std::unordered_map<std::string, types::type> type_mapping) : types(type_mapping) {}
		type_environment(const type_environment& other) : types(other.types) {}

		void set(const std::string& name, types::type type)
		{
			types.insert_or_assign(name, type);
		}

		types::type get(const std::string& name)
		{
			return types.at(name);
		}

	private:
		std::unordered_map<std::string, types::type> types;
	};

	class typechecker_stage : public language::typechecking_stage<extended_ast::node_p, extended_ast::node_p>
	{
	private:
		type_environment base_environment;

	public:
		typechecker_stage() {}
		typechecker_stage(type_environment environment) : base_environment(environment) {}

		extended_ast::node_p typecheck(extended_ast::node_p extended_ast) override
		{
			using namespace types;

			auto t_env = type_environment{base_environment};
			return std::get<0>(typecheck(std::move(extended_ast), std::move(t_env)));
		}

		std::tuple<extended_ast::node_p, type_environment> typecheck(extended_ast::node_p n, type_environment&& t_env)
		{
			using namespace types;

			if (auto tuple = dynamic_cast<extended_ast::tuple*>(n.get()))
			{
				auto new_node = std::make_unique<extended_ast::tuple>();
				auto new_type = product_type();

				for (decltype(auto) element : tuple->get_children())
				{
					extended_ast::node_p new_element;
					std::tie(new_element, t_env) = typecheck(std::move(element), std::move(t_env));
					new_type.product.push_back(new_element->type);
					new_node->add(std::move(new_element));
				}

				new_node->type = new_type;

				return std::make_tuple(
					std::move(new_node),
					std::move(t_env
					));
			}
			else if (auto id = dynamic_cast<extended_ast::identifier*>(n.get()))
			{
				id->type = t_env.get(id->name);
				return std::make_tuple(
					std::move(n),
					std::move(t_env)
				);
			}
			else if (auto assignment = dynamic_cast<extended_ast::assignment*>(n.get()))
			{
				// Typecheck value
				extended_ast::node_p new_value;
				std::tie(new_value, t_env) = typecheck(std::move(assignment->value), std::move(t_env));

				// Put id type in env
				t_env.set(assignment->id.name, new_value->type);
				assignment->id.type = new_value->type;

				return std::make_tuple(
					std::make_unique<extended_ast::assignment>(
						std::move(assignment->id),
						std::move(new_value)
						),
					std::move(t_env)
				);
			}
			else if (auto fc = dynamic_cast<extended_ast::function_call*>(n.get()))
			{
				extended_ast::node_p typechecked_params;
				std::tie(typechecked_params, t_env) = typecheck(
					std::move(std::make_unique<extended_ast::tuple>(std::move(fc->params))),
					std::move(t_env)
				);

				auto env_type = t_env.get(fc->id.name);

				function_type env_fn_type = std::get<5>(env_type);

				if (!(std::get<1>(typechecked_params->type) == env_fn_type.from))
				{
					throw std::runtime_error("Type error");
				}

				return std::make_tuple(
					std::make_unique<extended_ast::function_call>(
						std::move(fc->id),
						std::move(*dynamic_cast<extended_ast::tuple*>(typechecked_params.get())),
						env_fn_type.to
						),
					std::move(t_env)
				);
			}
			else if (auto integer = dynamic_cast<extended_ast::integer*>(n.get()))
			{
				integer->type = types::integer_type();
				return std::make_tuple(std::move(n), std::move(t_env));
			}
			else if (auto string = dynamic_cast<extended_ast::string*>(n.get()))
			{
				string->type = types::string_type();
				return std::make_tuple(std::move(n), std::move(t_env));
			}

			throw std::runtime_error("Unknown node type");
		}
	};
}