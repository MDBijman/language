#include "fe/data/types.h"
#include <string>

namespace fe
{
	namespace types
	{
		// Array

		array_type::array_type() : element_type(new unset()), count(0) {}
		array_type::array_type(unique_type t, size_t count) : element_type(std::move(t)), count(count) {}
		array_type::array_type(type& t, size_t count) : element_type(t.copy()), count(count) {}

		// Move

		array_type::array_type(array_type&& other) : element_type(std::move(other.element_type)), count(other.count) {}
		array_type& array_type::operator=(array_type&& other)
		{
			this->element_type = std::move(other.element_type);
			this->count = other.count;
			return *this;
		}

		// Copy

		array_type::array_type(const array_type& other) : element_type(other.element_type->copy()), count(other.count) {}
		array_type& array_type::operator=(const array_type& other)
		{
			this->element_type.reset(other.element_type->copy());
			this->count = other.count;
			return *this;
		}

		array_type::operator std::string() const
		{
			return "[" + std::string(*element_type) + "; " + std::to_string(count) + "]";
		}


		// Reference

		reference_type::reference_type() : referred_type(new unset()) {}
		reference_type::reference_type(unique_type t) : referred_type(std::move(t)) {}
		reference_type::reference_type(type& t) : referred_type(t.copy()) {}

		// Move

		reference_type::reference_type(reference_type&& other) : referred_type(std::move(other.referred_type)) {}
		reference_type& reference_type::operator=(reference_type&& other)
		{
			this->referred_type = std::move(other.referred_type);
			return *this;
		}

		// Copy

		reference_type::reference_type(const reference_type& other) : referred_type(other.referred_type->copy()) {}
		reference_type& reference_type::operator=(const reference_type& other)
		{
			this->referred_type.reset(other.referred_type->copy());
			return *this;
		}

		reference_type::operator std::string() const
		{
			return "&" + std::string(*referred_type);
		}


		// Sum

		sum_type::sum_type() {}
		sum_type::sum_type(std::vector<unique_type> sum) : sum(std::move(sum)) {}

		// Move
		sum_type::sum_type(sum_type&& other)
		{
			sum = std::move(other.sum);
		}
		sum_type& sum_type::operator=(sum_type&& other)
		{
			this->sum = std::move(other.sum);
			return *this;
		}

		// Copy
		sum_type::sum_type(const sum_type& other)
		{
			for (const auto& elem : other.sum)
			{
				sum.push_back(std::unique_ptr<type>(elem->copy()));
			}
		}
		sum_type& sum_type::operator=(const sum_type& other)
		{
			for (const auto& elem : other.sum)
			{
				sum.push_back(std::unique_ptr<type>(elem->copy()));
			}
			return *this;
		}

		sum_type::operator std::string() const
		{
			std::string r = "(";

			for (auto it = sum.begin(); it != sum.end(); ++it)
			{
				r.append(std::string(**it));

				if (it != sum.end() - 1)
				{
					r.append(" | ");
				}
			}

			r.append(")");
			return r;
		}

		size_t sum_type::index_of(std::string name)
		{
			for (int i = 0; i < sum.size(); i++)
			{
				if (dynamic_cast<nominal_type*>(sum[i].get())->name == name) return i;
			}
			assert(!"Type name is not in this sum type");
		}

		// Product

		product_type::product_type() {}
		product_type::product_type(std::vector<unique_type> product) : product(std::move(product)) {}

		// Move
		product_type::product_type(product_type&& other)
		{
			product = std::move(other.product);
		}
		product_type& product_type::operator=(product_type&& other)
		{
			this->product = std::move(other.product);
			return *this;
		}

		// Copy
		product_type::product_type(const product_type& other)
		{
			for (const auto& pair : other.product)
			{
				product.push_back(types::unique_type(pair->copy()));
			}
		}
		product_type& product_type::operator=(const product_type& other)
		{
			for (const auto& pair : other.product)
			{
				product.push_back(types::unique_type(pair->copy()));
			}
			return *this;
		}

		product_type::operator std::string() const
		{
			std::string r = "(";

			for (auto it = product.begin(); it != product.end(); ++it)
			{
				r.append(std::string(**it));

				if (it != product.end() - 1)
				{
					r.append(", ");
				}
			}

			r.append(")");
			return r;
		}

		// Function Type

		function_type::function_type(unique_type f, unique_type t) : from(std::move(f)), to(std::move(t)) {}
		function_type::function_type(const type& f, const type & t) : from(f.copy()), to(t.copy()) {}

		// Move
		function_type::function_type(function_type&& other)
		{
			from = std::move(other.from);
			to = std::move(other.to);
		}
		function_type& function_type::operator=(function_type&& other)
		{
			this->from = std::move(other.from);
			this->to = std::move(other.to);
			return *this;
		}

		// Copy
		function_type::function_type(const function_type& other)
		{
			from = types::make_unique(*other.from);
			to = types::make_unique(*other.to);
		}
		function_type& function_type::operator=(const function_type& other)
		{
			this->from = types::make_unique(*other.from);
			this->to = types::make_unique(*other.to);
			return *this;
		}

		function_type::operator std::string() const
		{
			std::string r = std::string(*from);
			r.append(" -> ");
			r.append(std::string(*to));
			return r;
		}

		// Nominal Type

		nominal_type::nominal_type(std::string name, unique_type inner) : name(name), inner(std::move(inner)) {}
		nominal_type::nominal_type(std::string name, const type& inner) : name(name), inner(inner.copy()) {}

		// Move
		nominal_type::nominal_type(nominal_type&& other) :
			name(std::move(other.name)), inner(std::move(other.inner)) {}
		nominal_type& nominal_type::operator=(nominal_type&& other)
		{
			this->name = std::move(other.name);
			this->inner = std::move(other.inner);
			return *this;
		}

		// Copy

		nominal_type::nominal_type(const nominal_type& other) :
			name(other.name), inner(other.inner->copy()) {}
		nominal_type& nominal_type::operator=(const nominal_type& other)
		{
			this->name = other.name;
			this->inner = types::make_unique(*other.inner);
			return *this;
		}

		nominal_type::operator std::string() const
		{
			std::string r = this->name + "(";
			r += inner->operator std::string();
			r += ")";
			return r;
		}


		// Operators

		bool operator==(const sum_type& one, const sum_type& two)
		{
			if (one.sum.size() != two.sum.size()) return false;

			for (unsigned int i = 0; i < one.sum.size(); i++)
			{
				if (!(*one.sum.at(i) == two.sum.at(i).get())) return false;
			}

			return true;
		}

		bool operator==(const product_type& one, const product_type& two)
		{
			if (one.product.size() != two.product.size()) return false;

			for (unsigned int i = 0; i < one.product.size(); i++)
			{
				if (!(*one.product.at(i) == two.product.at(i).get())) return false;
			}

			return true;
		}

		bool operator==(const function_type& one, const function_type& two)
		{
			return (*one.from == two.from.get()) && (*one.to == two.to.get());
		}

		bool operator==(const array_type& one, const array_type& two)
		{
			return (*one.element_type == two.element_type.get()) && (one.count == two.count);
		}

		bool operator==(const reference_type& one, const reference_type& two)
		{
			return *one.referred_type == two.referred_type.get();
		}

		bool operator==(const nominal_type& one, const nominal_type& two)
		{
			return (*one.inner == two.inner.get()) && (one.name == two.name);
		}
	}
}