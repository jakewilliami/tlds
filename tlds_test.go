package tlds

import "testing"

func TestMap(t *testing.T) {
	domain := ".aaa"
	aaaTld := Map[domain]
	if aaaTld.Domain != domain {
		t.Fatalf(`Expected %q, found %q`, domain, aaaTld.Domain)
	}
}
