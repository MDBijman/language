#pragma once
#include "fe/pipeline/lexer_stage.h"
#include "fe/pipeline/parser_stage.h"
#include "fe/pipeline/resolution_stage.h"
#include "fe/pipeline/typechecker_stage.h"
#include "fe/pipeline/lowering_stage.h"
#include "fe/pipeline/bytecode_gen_stage.h"
#include "fe/pipeline/linker_stage.h"
#include "fe/pipeline/bytecode_optimization_stage.h"
#include "fe/pipeline/bytecode_printing_stage.h"
#include "fe/data/bytecode.h"
#include "fe/pipeline/error.h"

#include <memory>
#include <tuple>
#include <variant>
#include <vector>
#include <thread>

namespace fe
{
	class pipeline 
	{
	public:
		pipeline() :
			lexer(lexing_stage{}),
			parser(parsing_stage{})
		{}

		ext_ast::ast parse(const std::string& code) 
		{
			auto res = lexer.lex(code);
			if (std::holds_alternative<lexing::error>(res))
				throw fe::parse_error{std::get<lexing::error>(res).message};
			auto parse_output = parser.parse(std::get<std::vector<lexing::token>>(res));

			if (std::holds_alternative<fe::parse_error>(parse_output))
				throw std::get<fe::parse_error>(parse_output);
			return std::move(std::get<fe::ext_ast::ast>(parse_output));
		}

		void typecheck(ext_ast::ast& ast) const
		{
			auto root = ast.root_id();
			auto& root_node = ast.get_node(root);
			ext_ast::resolve(root_node, ast);
			ext_ast::typecheck(root_node, ast);
		}

		core_ast::ast lower(ext_ast::ast& ast) const
		{
			return ext_ast::lower(ast);
		}

		vm::program generate(core_ast::ast& ast) const
		{
			return vm::generate_bytecode(ast);
		}

		void optimize_program(vm::program& e) const
		{
			vm::optimization_settings s;
			vm::optimize_program(e, s);
		}

		vm::executable link(vm::program& ast) const
		{
			return vm::link(ast);
		}

		void optimize_executable(vm::executable& e) const
		{
			vm::optimization_settings s;
			vm::optimize_executable(e, s);
		}

		void print_bytecode(const std::string& filename, vm::executable& e) const
		{
			vm::print_bytecode(filename, e);
		}

	private:
		lexing_stage lexer;
		parsing_stage parser;
	};
}