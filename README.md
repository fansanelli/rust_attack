My goal is to benchmark native rust speed against python implementation in RsaCtfTool (https://github.com/RsaCtfTool/RsaCtfTool).
This is a rust port, all credits goes to the original authors.

First test with "Shor" algorithm:

time ./target/release/rust_attack 
263209

real	0m6,143s
user	0m6,138s
sys	0m0,000s

time ./classical_shor.py 
(263209, 131101)

real	0m5,521s
user	0m5,511s
sys	0m0,007s

