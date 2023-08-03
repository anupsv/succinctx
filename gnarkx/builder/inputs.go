package builder

import (
	"github.com/succinctlabs/gnark-gadgets/vars"
)

// InputReader is used for reading inputs into a circuit that were provided at the time of the
// request, either on-chain or off-chain.
type InputReader struct {
	api   API
	ptr   int
	bytes []vars.Byte
}

// Creates a new InputReader.
func NewInputReader(api API, bytes []vars.Byte) *InputReader {
	return &InputReader{
		api:   api,
		ptr:   0,
		bytes: bytes,
	}
}

// Reads a single byte from the input stream.
func (r *InputReader) readByte() vars.Byte {
	out := r.bytes[r.ptr]
	r.ptr++
	return out
}

// Reads a byte32 from the input stream.
func (r *InputReader) ReadBytes32() [32]vars.Byte {
	var out [32]vars.Byte
	for i := 0; i < 32; i++ {
		out[i] = r.readByte()
	}
	return out
}