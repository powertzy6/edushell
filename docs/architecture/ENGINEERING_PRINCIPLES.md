# EduShell Engineering Principles

## General Principles

### E1. Modular Architecture
- Setiap komponen adalah modul independen dengan interface jelas
- Komunikasi antar modul via sinyal/slot atau IPC yang terdefinisi
- Modul dapat dikembangkan, diuji, dan diganti secara independen

### E2. Clean Architecture Layers
```
[UI Layer] → [Domain Layer] → [Infrastructure Layer]
            ↑                            |
            └── Repository Pattern ──────┘
```
- UI Layer: GTK4 widgets, panels, dialogs
- Domain Layer: business logic, models, use cases
- Infrastructure Layer: Cinnamon API calls, file I/O, settings backend

### E3. SOLID Principles
- **S**ingle Responsibility: Setiap class/module memiliki satu tanggung jawab
- **O**pen/Closed: Terbuka untuk ekstensi, tertutup untuk modifikasi
- **L**iskov Substitution: Subclass harus dapat menggantikan parent class
- **I**nterface Segregation: Interface spesifik, tidak generic
- **D**ependency Inversion: Bergantung pada abstraksi, bukan konkret

### E4. DRY (Don't Repeat Yourself)
- Logika bisnis ditulis sekali di Domain Layer
- Konstanta dan konfigurasi di centralized config
- Utility functions di shared library

### E5. KISS (Keep It Simple, Stupid)
- Solusi paling sederhana yang memenuhi requirements
- Jangan over-engineer untuk skenario yang belum ada
- Jika sebuah fungsi tidak bisa dijelaskan dalam 2 kalimat, terlalu kompleks

### E6. Separation of Concerns
- UI logic tidak boleh bercampur dengan business logic
- Business logic tidak boleh bergantung pada framework tertentu
- Konfigurasi tidak boleh hardcode di kode

### E7. Error Handling
- Setiap fungsi yang bisa gagal harus memiliki error handling
- Jangan pernah suppress error tanpa log
- User-facing error: friendly, actionable, non-technical
- Developer-facing error: detailed, logged, traceable

### E8. Logging
- Level: ERROR, WARN, INFO, DEBUG, TRACE
- Log ke file di `~/.local/share/edushell/logs/`
- Log rotasi: 7 hari, max 50MB
- Jangan log sensitive information (password, token)

### E9. Testing
- Unit test untuk Domain Layer
- Integration test untuk Infrastructure Layer
- UI test untuk komponen kritis
- Minimal coverage: 70% Domain Layer
- Test harus deterministic, tidak bergantung pada environment

### E10. Defensive Programming
- Validasi input di setiap boundary
- Jangan trust external data (file, API, argument)
- Fail fast dengan pesan jelas
- Handle edge cases secara eksplisit

## Language-Specific

### Vala (main shell components)
- Strict typing
- Gunakan signal/slot pattern untuk event
- Avoid global state
- Memory management: manual dengan reference counting

### Python (extensions, scripts)
- Type hints wajib
- F-strings untuk formatting
- Context manager untuk resource management
- Avoid dynamic attribute

### Rust (future components, v3+)
- No unsafe blocks unless absolutely necessary
- Proper error handling with Result/Option
- Zero-cost abstractions

## Dependency Management
- Setiap dependency harus memiliki alasan jelas
- Dokumentasikan lisensi setiap dependency
- Minimal dependency — evaluate before adding
- Pin version untuk reproducibility

## Code Review Standards
- Setiap PR harus melewati minimal 1 review
- CI harus green sebelum merge
- Tidak ada TODO atau FIXME di main branch
- Setiap fungsi baru harus memiliki test
