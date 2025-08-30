package tlds

// Generate new const library file with go generate
//
// Idea from Simon Sawert:
// https://github.com/bombsimon/tld-validator/blob/c0d0fbf9/tld.go#L9
//
//go:generate echo "[INFO] Generating library file"
//go:generate go run tools/writetlds/main.go const
//go:generate echo "[INFO] Generating JSON file"
//go:generate go run tools/writetlds/main.go json

// TLD types
// https://stackoverflow.com/a/71934535/
type Type string

const (
	Generic           Type = "generic"
	CountryCode       Type = "country-code"
	Sponsored         Type = "sponsored"
	Test              Type = "test"
	GenericRestricted Type = "generic-restricted"
	Infrastructure    Type = "infrastructure"
)

type TLD struct {
	Domain  string
	Type    Type
	Manager string
	Country string
}
