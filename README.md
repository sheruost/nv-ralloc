# [WIP] Ralloc

A recoverable lock-free memory allocator based on LRMalloc for NVM
architecture written in Rust.

## Note

The project is implemented from the academic paper "Understanding and
Optimizing Persistent Memory Allocation" by Wentao Cai, Haosen Wenn, 
H. Alan Beadle, Chris Kjellqvist, Mohammad Hedayati, and Michael 
L. Scott. The [paper] details a nonblocking allocator which provides 
recoverability, a correctness criterion for persistent allocators.

## Supporting Features

- [ ] Ralloc heap implementations
	- [ ] SuperBlock Region
	- [ ] Descriptor Region
	- [ ] Metadata Region
- [ ] standard APIs
	- [ ] recovery
	- [ ] alloc / deallocation
	- [ ] persistent root management
- [ ] filter garbage collections

## License

Planning for [GPLv3-or-later] after released.

[paper]: https://arxiv.org/pdf/2003.06718.pdf
[GPLv3-or-later]: https://spdx.org/licenses/GPL-3.0-or-later.html
