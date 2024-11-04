package finitefield

import (
	"errors"
)

// FiniteField represents a finite field of order p
type FiniteField struct {
	order int // The prime order of the field
}

// FieldElement represents an element in the finite field
type FieldElement struct {
	value int
	field *FiniteField
}

// NewFiniteField creates a new finite field of the given order
func NewFiniteField(order int) *FiniteField {
	if order <= 1 || !isPrime(order) {
		panic("Order must be a prime number greater than 1")
	}
	return &FiniteField{order: order}
}

// NewFieldElement creates a new field element, ensuring it's valid
func (ff *FiniteField) NewFieldElement(value int) (FieldElement, error) {
	if value < 0 || value >= ff.order {
		return FieldElement{}, errors.New("value must be in the range [0, order)")
	}
	return FieldElement{value: value, field: ff}, nil
}

func (fe1 FieldElement) Equals(fe2 FieldElement) bool {
	return fe1.value == fe2.value && fe1.field.order == fe2.field.order
}

func (fe1 FieldElement) Add(fe2 FieldElement) FieldElement {
	verify(fe1, fe2)

	return FieldElement{value: (fe1.value + fe2.value) % fe1.field.order, field: fe1.field}
}

func (fe1 FieldElement) Substract(fe2 FieldElement) FieldElement {
	verify(fe1, fe2)

	return FieldElement{value: (fe1.value - fe2.value) % fe1.field.order, field: fe1.field}
}

func (fe1 FieldElement) Multiply(fe2 FieldElement) FieldElement {
	verify(fe1, fe2)

	return FieldElement{value: (fe1.value * fe2.value) % fe1.field.order, field: fe1.field}
}

func (fe FieldElement) Exp(exp int) FieldElement {
	base := fe.value
	modulus := fe.field.order

	if exp < 0 {
		// Negative exponent: compute the modular inverse
		exp = -exp
		base = modInverse(base, modulus)
	}

	result := modExp(base, exp, modulus)
	return FieldElement{value: result, field: fe.field}
}

func (fe1 FieldElement) Div(fe2 FieldElement) FieldElement {

	modulus := fe1.field.order
	inverse := modInverse(fe2.value, modulus)
	result := (fe1.value * inverse) % modulus

	return FieldElement{value: result, field: fe1.field}
}

// Helper function to compute modular inverse
func modInverse(value, modulus int) int {
	return modExp(value, modulus-2, modulus) // Fermat's Little Theorem for prime modulus
}

// Helper function to compute modular exponentiation
func modExp(base, exp, modulus int) int {
	result := 1
	base = base % modulus

	for exp > 0 {
		if exp%2 == 1 {
			result = (result * base) % modulus
		}
		base = (base * base) % modulus
		exp /= 2
	}
	return result
}

func isPrime(n int) bool {
	if n <= 1 {
		return false
	}
	for i := 2; i*i <= n; i++ {
		if n%i == 0 {
			return false
		}
	}
	return true
}

func verify(fe1, fe2 FieldElement) {
	if fe1.field != fe2.field {
		panic("The field elements do not belong to the same finite field")
	}
}
