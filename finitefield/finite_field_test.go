package finitefield

import (
	"fmt"
	"testing"
)

func TestNewFiniteField(t *testing.T) {
	tests := []struct {
		order     int
		expectErr bool
	}{
		{2, false}, // valid prime
		{3, false}, // valid prime
		{4, true},  // not prime
		{1, true},  // not prime
		{0, true},  // not prime
	}

	for _, tt := range tests {
		t.Run(fmt.Sprintf("Order: %d", tt.order), func(t *testing.T) {
			if tt.expectErr {
				defer func() {
					if r := recover(); r == nil {
						t.Errorf("Expected panic for order %d", tt.order)
					}
				}()
				NewFiniteField(tt.order)
			} else {
				ff := NewFiniteField(tt.order)
				if ff.order != tt.order {
					t.Errorf("Expected order %d, got %d", tt.order, ff.order)
				}
			}
		})
	}
}

func TestNewFieldElement(t *testing.T) {
	ff := NewFiniteField(5)

	tests := []struct {
		value     int
		expectErr bool
	}{
		{0, false}, // valid
		{1, false}, // valid
		{4, false}, // valid
		{-1, true}, // invalid
		{5, true},  // invalid
	}

	for _, tt := range tests {
		t.Run(fmt.Sprintf("Value: %d", tt.value), func(t *testing.T) {
			fe, err := ff.NewFieldElement(tt.value)
			if tt.expectErr {
				if err == nil {
					t.Errorf("Expected error for value %d", tt.value)
				}
			} else {
				if err != nil {
					t.Errorf("Unexpected error for value %d: %v", tt.value, err)
				} else if fe.value != tt.value {
					t.Errorf("Expected value %d, got %d", tt.value, fe.value)
				}
			}
		})
	}
}

func TestFieldElementEquals(t *testing.T) {
	ff := NewFiniteField(7)
	fe1, _ := ff.NewFieldElement(3)
	fe2, _ := ff.NewFieldElement(3)
	fe3, _ := ff.NewFieldElement(4)

	if !fe1.Equals(fe2) {
		t.Errorf("fe1 should equal fe2")
	}

	if fe1.Equals(fe3) {
		t.Errorf("fe1 should not equal fe3")
	}
}

func TestFieldElementAdd(t *testing.T) {
	ff := NewFiniteField(5)
	fe1, _ := ff.NewFieldElement(2)
	fe2, _ := ff.NewFieldElement(3)
	result := fe1.Add(fe2)

	if result.value != 0 { // (2 + 3) % 5 = 0
		t.Errorf("Expected 0, got %d", result.value)
	}
}

func TestFieldElementSubtract(t *testing.T) {
	ff := NewFiniteField(5)
	fe1, _ := ff.NewFieldElement(3)
	fe2, _ := ff.NewFieldElement(1)
	result := fe1.Substract(fe2)

	if result.value != 2 { // (3 - 1) % 5 = 2
		t.Errorf("Expected 2, got %d", result.value)
	}
}

func TestFieldElementMultiply(t *testing.T) {
	ff := NewFiniteField(5)
	fe1, _ := ff.NewFieldElement(2)
	fe2, _ := ff.NewFieldElement(4)
	result := fe1.Multiply(fe2)

	if result.value != 3 { // (2 * 4) % 5 = 3
		t.Errorf("Expected 3, got %d", result.value)
	}
}

func TestFieldElementDiv(t *testing.T) {
	field := NewFiniteField(31) // Field with prime order 31

	fe1, _ := field.NewFieldElement(3)
	fe2, _ := field.NewFieldElement(24)
	result := fe1.Div(fe2)

	if result.value != 4 {
		t.Errorf("Expected 4, got %d", result.value)
	}
}

func TestFieldElementExp(t *testing.T) {
	field := NewFiniteField(31) // Field with prime order 31

	fe1, _ := field.NewFieldElement(17)
	exp := -3
	result := fe1.Exp(exp)

	if result.value != 29 {
		t.Errorf("Expected 29, got %d", result.value)
	}
}
