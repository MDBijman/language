#pragma once
#include "types.h"
#include "extended_ast.h"
#include <unordered_map>
#include <string>

namespace fe
{
	class typecheck_environment
	{
	public:
		typecheck_environment() {}
		typecheck_environment(std::unordered_map<std::string, types::type> type_mapping) : types(type_mapping) {}
		typecheck_environment(const typecheck_environment& other) : types(other.types), name(other.name), namespaces(other.namespaces) {}

		void add_module(typecheck_environment&& other)
		{
			// TODO fix issue of setting the module name after a module (with that name) -> namespaces are not merged
			if (other.name.has_value() && other.name.value() != this->name)
			{
				auto existing_namespace_location = namespaces.find(other.name.value());

				if (existing_namespace_location != namespaces.end())
					// Merge module with the existing one
				{
					other.name = std::optional<std::string>();
					existing_namespace_location->second.add_module(std::move(other));
				}
				else
				{
					namespaces.insert({ other.name.value(), other });
				}
			}
			else
			{
				types.insert(std::make_move_iterator(other.types.begin()), std::make_move_iterator(other.types.end()));
			}
		}

		void set_type(const std::string& id, types::type type)
		{
			types.insert_or_assign(id, type);
		}

		void set_type(const extended_ast::identifier& id, types::type type)
		{
			if (id.segments.size() == 1)
			{
				types.insert_or_assign(id.segments.at(0), type);
			}
			else
			{
				namespaces.find(id.segments.at(0))->second.set_type(id.without_first_segment(), type);
			}
		}

		const types::type& typeof(const extended_ast::identifier& id) const
		{
			if (id.segments.size() == 1)
			{
				return types.at(id.segments.at(0));
			}
			else
			{
				const types::type* type = nullptr;

				for (int i = 0; i < id.segments.size(); i++)
				{
					if (namespaces.find(id.segments.at(i)) != namespaces.end())
					{
						return namespaces.find(id.segments.at(0))->second.typeof(id.without_first_segment());
					}

					type = &types.at(id.segments.at(i));

					if (i == id.segments.size() - 1)
						return *type;

					auto& product_type = std::get<types::product_type>(*type);
					auto type_location = std::find_if(product_type.product.begin(), product_type.product.end(), [&](const std::pair<std::string, types::type>& x) {
						return x.first == id.segments.at(i + 1);
					});

					type = &type_location->second;
				}
			}
		}

		void build_access_pattern(extended_ast::identifier& id, int index = 0)
		{
			if (namespaces.find(id.segments.at(index)) != namespaces.end())
				namespaces.find(id.segments.at(index))->second.build_access_pattern(id, index + 1);
			else
			{
				auto variable_name = id.segments.at(index);

				std::reference_wrapper<types::type> current_type = types.at(variable_name);
				for (int i = index + 1; i < id.segments.size(); i++)
				{
					auto& product_type = std::get<types::product_type>(current_type.get());
					auto next_loc = std::find_if(product_type.product.begin(), product_type.product.end(), [&](auto& x) {
						return x.first == id.segments.at(i);
					});

					id.offsets.push_back(std::distance(product_type.product.begin(), next_loc));
				}
			}
		}

		std::string to_string(bool include_modules = false)
		{
			auto indent = [](std::string& text) {
				return std::regex_replace(text, std::regex("\\n"), "\n\t");
			};

			std::string r = name.has_value() ?
				"type_environment: " + name.value() + " (" :
				"type_environment (";

			uint32_t counter = 0;
			for (auto& pair : types)
			{
				std::string t = "\n\t" + pair.first + ": ";
				t.append(std::visit(types::to_string, pair.second));
				r.append(t);
				r.append(",");
			}

			if (include_modules)
			{
				r.append(indent("\n" + std::string("modules (")));
				for (auto& string_namespace_pair : namespaces)
				{
					r.append(indent(indent(
						"\n" + string_namespace_pair.second.to_string(false) + ","
					)));
				}
				r.append("\n\t)");
			}

			r.append("\n)");
			return r;
		}

		std::optional<std::string> name;

	private:
		std::unordered_map<std::string, types::type> types;
		std::unordered_multimap<std::string, typecheck_environment> namespaces;
	};
}
