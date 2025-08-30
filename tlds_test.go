package tlds

import "testing"

func TestTLDMap(t *testing.T) {
	domain := ".aaa"
	aaaTld := TLDMap[domain]
	if aaaTld.Domain != domain {
		t.Fatalf(`Expected %q, found %q`, domain, aaaTld.Domain)
	}
}
