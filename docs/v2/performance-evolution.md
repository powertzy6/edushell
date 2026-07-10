# Performance Evolution Plan — EduShell v2.0+

## Targets (Celeron N3060 + 4GB RAM)
| Metric | Current | Target | 
|--------|---------|--------|
| Idle RAM | < 256 MB | < 180 MB |
| Cold start | < 5 s | < 2 s |
| Search latency | < 200 ms | < 50 ms |
| Panel render | < 100 ms | < 30 ms |
| Theme switch | < 500 ms | < 100 ms |

## v2.0
- [x] All modules are library crates (no redundant binaries)
- [x] Config engine uses HashMap (O(1) lookup)
- [x] Search engine truncates results (no unbounded queries)

## v2.2
- [ ] Lazy loading: modules load on demand, not at startup
- [ ] Thread pool sizing: match CPU core count (N3060 = 2 cores)
- [ ] Shared event channel (tokio broadcast) instead of per-module polling
- [ ] CSS generation caching (invalidate only on token change)

## v2.5
- [ ] Indexed search (tantivy or similar for full-text)
- [ ] Icon cache with LRU eviction
- [ ] Pre-compiled theme binary (skip CSS parse at runtime)
- [ ] Startup profile-guided optimization (PGO)

## v3.0
- [ ] GPU-accelerated compositing (kmscube/drm backend)
- [ ] Zero-copy IPC between shell components
- [ ] Memory-mapped resource files (no read overhead)
- [ ] Async I/O for disk operations (tokio)

## v4.0+
- [ ] Custom Wayland compositor with Vulkan renderer
- [ ] Ahead-of-time (AOT) compiled widgets
- [ ] Predictive search indexing (idle-time background index)
