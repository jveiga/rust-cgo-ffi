package main

/*
#cgo LDFLAGS: ./lib/libffi_example.a -ldl
#include "./lib/ffi-example.h"
*/
import (
	"C"
)
import "fmt"

func main() {
	// var n C.int
	// C.add_get(&n)
	// C.add_get(&n)
	p := C.call()
	C.inc(p)
	// p.Inc()
	fmt.Printf("%+v\n", p)
	// fmt.Println(int(n))
}
