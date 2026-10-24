# Platform support matrix

| Target | Role | Status |
|---|---|---|
| Linux x86_64 | API, worker, web build, PostgreSQL host | Primary self-host target |
| macOS Apple Silicon | Development, desktop, API | Expected; verify native toolchain |
| Windows x86_64 | Development and desktop | Expected; verify packaging |
| WebAssembly browser | Web UI | Dioxus web target; verify with current Dioxus CLI |
| Android | Mobile UI | Planned; Android SDK/NDK and Dioxus mobile toolchain required |
| iOS | Mobile UI | Planned; macOS/Xcode required |

Do not claim every target builds until its CI job actually runs. The core backend and data model must not depend on UI target features.
