# Latios

A minimal productivity TUI (Terminal User Interface) for managing todos with rich context for AI tool consumption.

## Features

- **Todo Management**: Add, complete, and delete tasks
- **Rich Context**: Attach descriptions, file references, code snippets, and tags to tasks
- **Projects**: Organize tasks into projects (coming soon)
- **AI Export**: Export tasks to markdown format optimized for AI tools
- **Persistent Storage**: All data stored in human-readable JSON

## Installation

```bash
cargo build --release
```

## Usage

Run the application:

```bash
cargo run
```

Or use the compiled binary:

```bash
./target/release/latios
```

## Keybindings

### Normal Mode (Task List)

- `j`/`k` or `↓`/`↑` - Navigate tasks
- `Space`/`Enter` - Toggle task completion
- `a` - Add new task
- `e` - Edit task details (coming soon)
- `d` - Delete task
- `p` - Switch projects (coming soon)
- `x` - Export to markdown
- `?` - Show help
- `q` - Quit

### Insert Mode

- `ESC` - Cancel input
- `Enter` - Confirm input
- Arrow keys - Move cursor
- `Backspace` - Delete character

## Data Storage

Tasks are automatically saved to `./data/latios.json` when you quit the application.

The JSON file is human-readable and can be manually edited if needed.

## Markdown Export

Press `x` to export your tasks to a timestamped markdown file (e.g., `latios-export-20241215-103045.md`).

The export format includes:
- Task metadata (ID, status, timestamps, project, tags)
- Descriptions
- File references with line numbers
- Code snippets with syntax highlighting hints

This format is optimized for pasting into AI chat tools like Claude or ChatGPT to get help with your tasks.

## Roadmap

### Currently Implemented (MVP)
- ✅ Basic task management (add, complete, delete)
- ✅ Task navigation
- ✅ JSON persistence
- ✅ Markdown export
- ✅ Help screen

### Coming Soon
- 📋 Task detail view with rich context editing
  - Add/edit descriptions
  - Attach file references
  - Include code snippets
  - Tag management
- 📁 Project management and filtering
- 🔍 Search and filter tasks
- ⚡ More UI improvements

## Architecture

```
src/
├── main.rs           # Entry point and terminal setup
├── app.rs            # Application state machine
├── input.rs          # Keyboard input handling
├── models/           # Data models
│   ├── task.rs       # Task with rich context
│   ├── project.rs    # Project grouping
│   └── app_data.rs   # Root data container
├── storage/          # Persistence layer
│   ├── json.rs       # JSON load/save
│   └── export.rs     # Markdown export
└── ui/               # User interface
    ├── task_list.rs  # Main task list view
    ├── task_detail.rs # Task editing (coming soon)
    ├── project_list.rs # Project selector (coming soon)
    └── help.rs       # Help screen
```

## License

MIT
