# Gyst-TUI Project Specification

## Overview

Gyst-TUI is a Terminal User Interface (TUI) and Command Line Interface (CLI) application for managing todos. It provides both a graphical terminal interface and command-line tools for task management, with features like task creation, editing, deletion, and organization into groups.

## Core Features

-   Task Management (Create, Edit, Delete)
-   Task Organization (Groups)
-   Due Dates
-   Repeating Tasks
-   Task Notes
-   URL Links
-   Task Completion Status
-   Configurable UI (Icons, Colors, Keybindings)

## System Architecture

### Core Components

#### 1. Task Management (`src/task.rs`)

-   Defines the core `Task` struct
-   Handles task properties: name, date, repeats, group, description, URL, completion status
-   Implements task serialization/deserialization

#### 2. Configuration System (`src/configuration.rs`)

-   Manages user preferences and settings
-   Handles:
    -   Date formats
    -   UI icons
    -   Color schemes
    -   Keybindings
    -   Task display preferences
-   Configuration file locations:
    -   Unix: `~/.config/todui/settings.json`
    -   Windows: `C:\Users\<user>\AppData\Roaming\todui\settings.json`

#### 3. User Interface (`src/ui/`)

-   **Main UI Module** (`mod.rs`): Core UI components and layout management
-   **All Tasks Page** (`all_tasks_page.rs`): Main task list view
-   **Task Page** (`task_page.rs`): Individual task view/editing
-   **Delete Task Page** (`delete_task_page.rs`): Task deletion confirmation

#### 4. Task Form (`src/task_form.rs`)

-   Handles task creation and editing forms
-   Manages input validation and data formatting

#### 5. CLI Interface (`src/cli/`)

-   Command-line interface implementation
-   Commands:
    -   `ls`: List tasks
    -   `add`: Add new task
    -   `delete`: Remove task
    -   `complete`: Toggle task completion
    -   `config`: Manage settings
    -   `help`: Display help information

### Key Files and Their Purposes

1. `src/main.rs`

    - Application entry point
    - CLI argument parsing
    - Initialization of TUI or CLI mode

2. `src/app.rs`

    - Core application state management
    - Task data handling
    - Application lifecycle management

3. `src/utils.rs`

    - Utility functions
    - Helper methods for common operations

4. `src/repeat.rs`

    - Repeating task logic
    - Schedule management

5. `src/day_of_week.rs`
    - Day of week handling
    - Date-related utilities

### Data Flow

1. **Task Creation Flow**:

    - User initiates task creation (UI or CLI)
    - Task form collects data
    - Data validation
    - Task storage
    - UI update

2. **Task Management Flow**:

    - Task selection
    - Action execution (edit/delete/complete)
    - State update
    - UI refresh

3. **Configuration Flow**:
    - User modifies settings
    - Configuration validation
    - Settings storage
    - UI update

## Technical Implementation Details

### Key Data Structures

1. **Task Structure**:

```rust
struct Task {
    id: u32,
    name: String,
    date: Option<DateTime<Local>>,
    repeats: Option<Repeat>,
    group: Option<String>,
    description: Option<String>,
    url: Option<String>,
    complete: bool
}
```

2. **Configuration Structure**:

```rust
struct Configuration {
    date_formats: DateFormats,
    show_complete: bool,
    current_group: Option<String>,
    icons: Icons,
    colors: Colors,
    keybindings: Keybindings
}
```

### UI Implementation

The UI is built using a TUI framework with the following key components:

1. **Main View**:

    - Task list
    - Group navigation
    - Status bar

2. **Task Form**:

    - Input fields
    - Date picker
    - Group selector
    - URL input
    - Description editor

3. **Navigation**:
    - Vim-like keybindings
    - Mode switching (normal/insert)
    - Group navigation

### CLI Implementation

The CLI provides programmatic access to todos with the following features:

1. **Command Structure**:

    - Subcommands for different operations
    - Format options (JSON, text)
    - Filtering capabilities

2. **Output Formats**:
    - JSON for programmatic use
    - Formatted text for human reading

## Extension Points

The system is designed to be extensible in several ways:

1. **New Task Properties**:

    - Add new fields to the Task struct
    - Update UI components
    - Modify serialization

2. **UI Customization**:

    - Add new views
    - Modify existing layouts
    - Add new interaction modes

3. **CLI Commands**:

    - Add new subcommands
    - Extend existing commands
    - Add new output formats

4. **Integration Points**:
    - External calendar systems
    - Notification systems
    - Task synchronization

## Dependencies

-   Rust standard library
-   TUI framework
-   Date/time handling libraries
-   JSON serialization/deserialization
-   Optional: Nerd Fonts for enhanced icons

## Configuration Options

### Date Formats

-   Display formats
-   Input formats
-   Hints

### UI Customization

-   Icons
-   Colors
-   Keybindings

### Display Options

-   Show/hide completed tasks
-   Current group selection
-   Task grouping preferences
