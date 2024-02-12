// rank - easy
// source - exercism
package main

import (
	"fmt"
	"math"
	"strconv"
)

func armstrongNumbers(num int) bool {
	numString := strconv.FormatInt(int64(num), 10)
	sum := 0
	stringLength := len(numString)

	for i := 0; i < stringLength; i++ {
		convertedDigit, err := strconv.ParseInt(string(numString[i]), 10, 64)
		if err != nil {
			fmt.Println("Error converting string to int:", err)
		} else {
			pow := int64(math.Pow(float64(convertedDigit), float64(stringLength)))
			sum = int(int64(sum) + pow)
		}
	}

	return num == sum
}
