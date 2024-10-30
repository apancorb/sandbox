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
