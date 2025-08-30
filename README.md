<h1 align="center">Serialised TLDs in Go</h1>

A small library that provides serialised basic information about top level domains (TLDs).

An additional command-line utility, `tldinfo`, is provided.

---

## Quick Start

### Using the Library

Get the module:
```bash
$ go get github.com/jakewilliami/tlds
```

Basic library usage:
```go
package main

import (
	"fmt"

	"github.com/jakewilliami/tlds"
)

func main() {
	tld := tlds.TLDMap[".jp"]
	country := tld.Country
	fmt.Printf("%v\n", country)  // Japan
}
```

Types:
```go
type TLD struct {
	Domain  string
	Type    TLDType
	Manager string
	Country string
}

const (
	Generic           TLDType = "generic"
	CountryCode       TLDType = "country-code"
	Sponsored         TLDType = "sponsored"
	Test              TLDType = "test"
	GenericRestricted TLDType = "generic-restricted"
	Infrastructure    TLDType = "infrastructure"
)
```

### Using the CLI

Compile:
```bash
$ ./build.sh    # All-in-one build script, including updating generated const file
$ go build -o ./tldinfo cmd/tldinfo/main.go  # Or build it into a binary
```

Run:
```bash
$ ./tldinfo jp  # or ./tldinfo .jp
$ ./tldinfo -h  # help command coming soon™!
```

## Similar Projects

This library/CLI is **not** a top-level domain parser.  For parsers, see [`github.com/jpillora/go-tld`](https://github.com/jpillora/go-tld)/[`golang.org/x/net/publicsuffix`](https://pkg.go.dev/golang.org/x/net/publicsuffix)/[`github.com/bobesa/go-domain-util`](https://github.com/bobesa/go-domain-util).

This project is more similar to a top-level domain *validator* such as [`github.com/bombsimon/tld-validator`](https://github.com/bombsimon/tld-validator), which looks like a very good project, even with unicode support using the [`golang.org/x/net` IDNA submodule](https://pkg.go.dev/golang.org/x/net/idna).  Simon's TLD validator even generates a constant/static list of TLDs for library/offline use.  However, this package does not retain any of the other TLD information [defined by the IANA](https://www.iana.org/domains/root/db) as it pulls from [a different source](https://data.iana.org/TLD/tlds-alpha-by-domain.txt) to us.  Ultimately, Simon's TLD validator provides a different functionality to ours, but I really recommend checking it out as it seems to be well-written and you may be after simple validation, in which case [`tld-validator`](https://github.com/bombsimon/tld-validator) may be the best package for you.

Another tool that deals with ISO-3166 in a much broader scope is [`github.com/biter777/countries`](https://github.com/biter777/countries).  It primarily deals with country codes, but also has support for country codes.

## Caveats

  - Currently only supports country code TLDs (ccTLDs), as this is my personal main usecase for the application.  It does not yet generic TLDs (gTLD).
  - Internationalised ccTLDs are not yet supported.

## Project Structure

Library files are in the root directory.  Command-line tools are in the `cmd/` subdirectory.  The primary command line tool at time of writing simply states the country associated with a TLD.  Internal tools are in the `tools/` subdirectory (these are tools that help with package development).

## Citation

If your research depends on `tlds`, please consider giving us a formal citation: [`citation.bib`](./citation.bib)
