#pragma once
#include <vector>
#include <string>
#include <optional>
#include <variant>
#include <unordered_map>
#include <assert.h>
#include <array>

#include "fe/data/name_scope.h"
#include "fe/data/type_scope.h"
#include "fe/data/ast_data.h"
#include "utils/memory/data_store.h"

namespace fe::ext_ast
{
	enum class node_type
	{
		ASSIGNMENT,
		TUPLE,
		BLOCK,
		BLOCK_RESULT,
		FUNCTION,
		WHILE_LOOP,
		IF_STATEMENT,
		MATCH_BRANCH,
		MATCH,
		IDENTIFIER,
		FUNCTION_CALL,
		MODULE_DECLARATION,
		EXPORT_STMT,
		IMPORT_DECLARATION,
		DECLARATION,
		REFERENCE,
		ARRAY_VALUE,

		// Literals
		STRING,
		BOOLEAN,
		NUMBER,

		// Type declarations
		TYPE_DEFINITION,
		RECORD,
		RECORD_ELEMENT,
		IDENTIFIER_TUPLE,

		// Type expressions
		TYPE_TUPLE,
		TYPE_ATOM,
		FUNCTION_TYPE,
		REFERENCE_TYPE,
		ARRAY_TYPE,

		// (Math) operators
		ADDITION,
		SUBTRACTION,
		MULTIPLICATION,
		DIVISION,
		MODULO,
		EQUALITY,
		GREATER_THAN,
		GREATER_OR_EQ,
		LESS_THAN,
		LESS_OR_EQ,
	};

	constexpr bool is_binary_op(node_type kind)
	{
		return (kind == node_type::ADDITION
			|| kind == node_type::SUBTRACTION
			|| kind == node_type::MULTIPLICATION
			|| kind == node_type::DIVISION
			|| kind == node_type::MODULO
			|| kind == node_type::EQUALITY
			|| kind == node_type::GREATER_OR_EQ
			|| kind == node_type::GREATER_THAN
			|| kind == node_type::LESS_OR_EQ
			|| kind == node_type::LESS_THAN);
	}

	struct node
	{
		node() {}
		node(node_id id, node_type t) :
			id(id), kind(t) {}
		node(node_id id, node_type t, data_index i) :
			id(id), kind(t), data_index(i) {}
		node(node_id id, node_type t, data_index i, std::vector<node_id> children) :
			id(id), kind(t), data_index(i), children(children) {}
		node(node_id id, node_type t, std::vector<node_id> children) :
			id(id), kind(t), children(children) {}

		node_type kind;
		node_id id;
		std::vector<node_id> children;
		std::optional<node_id> parent_id;

		std::optional<types::unique_type> type;
		std::optional<data_index> data_index;
		std::optional<scope_index> name_scope_id;
		std::optional<scope_index> type_scope_id;
	};
}


namespace fe::ext_ast
{
	class ast
	{
		memory::dynamic_store<node> nodes;
		memory::dynamic_store<name_scope> name_scopes;
		memory::dynamic_store<type_scope> type_scopes;

		// Storage of node data
		memory::dynamic_store<identifier> identifiers;
		memory::dynamic_store<boolean> booleans;
		memory::dynamic_store<string> strings;
		memory::dynamic_store<number> numbers;

		node_id root;

	public:
		ast(node_type t)
		{
			root = nodes.create();
			nodes.get_at(root) = node(root, t);
			nodes.get_at(root).data_index = create_node_data(t);
			nodes.get_at(root).name_scope_id = create_name_scope();
			nodes.get_at(root).type_scope_id = create_type_scope();
		}

		// Root node
		node_id root_id()
		{
			return root;
		}

		// Nodes
		node_id create_node(node_type t)
		{
			auto new_node = nodes.create();
			get_node(new_node).id = new_node;
			get_node(new_node).kind = t;
			get_node(new_node).data_index = create_node_data(t);
			return new_node;
		}

		node& get_node(node_id id)
		{
			return nodes.get_at(id);
		}

		std::optional<identifier> get_module_name()
		{
			auto module_dec_id = find_node(node_type::MODULE_DECLARATION);
			if (!module_dec_id.has_value()) return std::nullopt;
			auto& module_dec_node = get_node(module_dec_id.value());
			auto& id_node = get_node(module_dec_node.children[0]);
			return get_data<identifier>(id_node.data_index.value());
		}

		std::optional<std::vector<identifier>> get_imports()
		{
			auto import_dec_id = find_node(node_type::IMPORT_DECLARATION);
			if (!import_dec_id.has_value()) return std::nullopt;
			auto& module_dec_node = get_node(import_dec_id.value());

			std::vector<identifier> imports;
			for (auto child : module_dec_node.children)
			{
				auto& id_node = get_node(child);
				imports.push_back(get_data<identifier>(id_node.data_index.value()));
			}
			return imports;
		}

		// Scopes
		scope_index create_name_scope()
		{
			return name_scopes.create();
		}

		scope_index create_name_scope(scope_index parent)
		{
			auto new_scope = name_scopes.create();
			name_scopes.get_at(new_scope).set_parent(parent);
			return new_scope;
		}

		name_scope& get_name_scope(scope_index id)
		{
			return name_scopes.get_at(id);
		}

		name_scope::get_scope_cb name_scope_cb()
		{
			return [&](scope_index i) { return &name_scopes.get_at(i); };
		}

		scope_index create_type_scope()
		{
			return type_scopes.create();
		}

		scope_index create_type_scope(scope_index parent)
		{
			auto new_scope = type_scopes.create();
			type_scopes.get_at(new_scope).set_parent(parent);
			return new_scope;
		}

		type_scope& get_type_scope(scope_index id)
		{
			return type_scopes.get_at(id);
		}

		type_scope::get_scope_cb type_scope_cb()
		{
			return [&](scope_index i) { return &type_scopes.get_at(i); };
		}

		// Node data 
		template<class DataType>
		DataType& get_data(data_index i);
		template<> identifier& get_data<identifier>(data_index i) { return identifiers.get_at(i); }
		template<> boolean&    get_data<boolean>(data_index i) { return booleans.get_at(i); }
		template<> string&     get_data<string>(data_index i) { return strings.get_at(i); }
		template<> number&     get_data<number>(data_index i) { return numbers.get_at(i); }

	private:
		std::optional<data_index> create_node_data(node_type t)
		{
			switch (t)
			{
			case node_type::IDENTIFIER: return identifiers.create();
			case node_type::NUMBER:     return numbers.create();
			case node_type::STRING:     return strings.create();
			case node_type::BOOLEAN:    return booleans.create();
			default:
				if (is_binary_op(t)) return strings.create();
				return std::nullopt;
			}
		}

		std::optional<node_id> find_node(node_type t)
		{
			for (node_id i = 0; i < nodes.get_data().size(); i++)
			{
				if (!nodes.is_occupied(i))
					continue;
				if (nodes.get_at(i).kind == t)
					return i;
			}
			return std::nullopt;
		}
	};
}
