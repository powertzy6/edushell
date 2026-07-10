# Hardware Targets — EduShell v1

## Target Hardware Specification

### Baseline (Minimum)
- **CPU**: Intel Celeron N3060 (2 core/2 thread @ 1.6 GHz) atau AMD A4-6210
- **RAM**: 4 GB DDR3L
- **Storage**: 128 GB SSD (eMMC acceptable but not recommended)
- **GPU**: Intel HD Graphics 400 (Braswell) / AMD Radeon R2
- **Display**: 1366 × 768 @ 60 Hz
- **Network**: WiFi 802.11n, Ethernet 10/100

### Recommended
- **CPU**: Intel Core i3-6100U (2 core/4 thread @ 2.3 GHz) atau AMD Ryzen 3 2200U
- **RAM**: 8 GB DDR4
- **Storage**: 256 GB SSD (NVMe preferred)
- **GPU**: Intel HD Graphics 520 / AMD Radeon Vega 3
- **Display**: 1920 × 1080 @ 60 Hz
- **Network**: WiFi 802.11ac, Ethernet Gigabit

### Premium
- **CPU**: Intel Core i5-1135G7 / AMD Ryzen 5 5500U
- **RAM**: 16 GB DDR4
- **Storage**: 512 GB NVMe SSD
- **GPU**: Intel Iris Xe / AMD Radeon Vega 7
- **Display**: 1920 × 1080 @ 120 Hz atau 2560 × 1440
- **Network**: WiFi 6, Ethernet Gigabit

## Chromebook Equivalents (for reference)
- **Minimum**: Intel Celeron N4020, 4GB RAM — setara Chromebook entry-level
- **Recommended**: Intel Pentium Silver N5030, 8GB RAM

## Notes
- Tidak ada dukungan untuk arsitektur 32-bit (x86) di v1
- Dukungan ARM64 (aarch64) untuk Raspberry Pi dan Chromebook dimulai di v2
- GPU dedicated (NVIDIA) didukung via open-source driver nouveau — tidak ada dukungan driver proprietary di v1
