# Post-Judgment Interest Calculator

A cross-platform desktop application for calculating post-judgment interest rates for federal and state jurisdictions in the United States. Built with Tauri, Rust, React, and TypeScript.

## Features

- **Federal Rate Calculations**: Automatically fetches real-time federal interest rates from the FRED API (1-Year Treasury Constant Maturity Rate)
- **State Rate Support**: Pre-populated database with interest rates for all 50 states plus DC
- **Database Management**: Built-in UI for viewing, editing, adding, and deleting state interest rates
- **Simple Interest Calculation**: Uses the formula: Principal × Rate × (Days / 365)
- **Date Range Support**: Calculate interest for any date range
- **API Key Management**: Secure local storage of FRED API key through the Settings tab
- **Cross-Platform**: Runs on Windows, macOS, and Linux (Windows MSI installer available)

## Screenshots

The application features three main tabs:

1. **Calculator Tab**: Input judgment details and calculate interest
2. **Database Manager Tab**: Manage state interest rates with an editable grid
3. **Settings Tab**: Configure your FRED API key

## Tech Stack

### Backend (Rust)
- **Tauri**: Desktop application framework
- **SQLite**: Local database via `rusqlite`
- **reqwest**: HTTP client for FRED API calls
- **chrono**: Date handling and calculations
- **serde**: JSON serialization/deserialization
- **thiserror**: Error handling
- **dotenvy**: Environment variable management

### Frontend (TypeScript/React)
- **React**: UI framework with functional components and hooks
- **TypeScript**: Type-safe JavaScript
- **Vite**: Build tool and dev server
- **Tailwind CSS**: Utility-first CSS framework
- **React DatePicker**: Date selection components
- **Tauri API**: Communication with Rust backend

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (v18 or higher)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Setup

1. Clone the repository:
```bash
git clone https://github.com/matt793/JudgmentRateDesktop.git
cd JudgmentRateDesktop
```

2. Install dependencies:
```bash
npm install
```

3. Get a FRED API key:
   - Visit [FRED API Key Registration](https://fred.stlouisfed.org/docs/api/api_key.html)
   - Create a free account or sign in
   - Request an API key (instant approval)
   - Save the key - you'll add it through the app's Settings tab

4. Run the development server:
```bash
npm run tauri dev
```

5. Configure your API key:
   - Launch the application
   - Go to the Settings tab
   - Paste your FRED API key and click "Save API Key"

## Building

To build the application for production:

```bash
npm run tauri build
```

This will create:
- Windows: MSI installer in `src-tauri/target/release/bundle/msi/`
- macOS: DMG and app bundle
- Linux: AppImage and deb packages

## Project Structure

```
JudgmentRateDesktop/
├── src/                         # React frontend source
│   ├── components/              # React components
│   │   ├── CalculatorTab.tsx    # Main calculator UI
│   │   ├── DbManagerTab.tsx     # Database management UI
│   │   └── SettingsTab.tsx      # Settings and API key config
│   ├── App.tsx                  # Main app component
│   ├── types.ts                 # TypeScript type definitions
│   └── main.tsx                 # React entry point
├── src-tauri/                   # Rust backend source
│   ├── src/
│   │   ├── main.rs              # Tauri app entry point
│   │   ├── lib.rs               # Library configuration
│   │   ├── commands.rs          # Tauri command handlers
│   │   ├── models.rs            # Data structures
│   │   ├── db.rs                # SQLite database operations
│   │   ├── rate_fetcher.rs      # FRED API integration
│   │   └── calculator.rs        # Interest calculation logic
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri configuration
├── package.json                 # Node.js dependencies
├── vite.config.ts               # Vite configuration
├── tailwind.config.js           # Tailwind CSS configuration
└── tsconfig.json                # TypeScript configuration
```

## How It Works

### Federal Rate Calculation
1. When calculating federal post-judgment interest:
   - The app determines the week preceding the judgment date (Monday to Sunday)
   - Fetches daily 1-Year Treasury yields for that week from FRED API
   - Calculates the average of valid yields (skipping missing data)
   - Converts the percentage to a decimal rate

### State Rate Handling
1. State rates are stored in a local SQLite database
2. Rates can be:
   - **Fixed**: A specific percentage rate
   - **Variable**: Federal rate plus an additional percentage
3. All rates are editable through the Database Manager tab

### Interest Calculation
- Uses simple interest formula: Interest = Principal × Rate × (Days / 365)
- Days are calculated inclusively between start and end dates
- Results include a disclaimer about consulting legal advice

## Database Schema

The SQLite database contains a single `state_rates` table:

```sql
CREATE TABLE state_rates (
    id INTEGER PRIMARY KEY,
    state TEXT NOT NULL UNIQUE,
    rate REAL NOT NULL,
    is_variable INTEGER NOT NULL DEFAULT 0,
    plus_percentage REAL DEFAULT 0,
    update_frequency TEXT,
    last_update TEXT,
    notes TEXT
);
```

## Development

### Running Tests
```bash
# Rust tests
cd src-tauri
cargo test

# Frontend tests (if implemented)
npm test
```

### Adding New States
Use the Database Manager tab to add new jurisdictions or modify existing rates.

### Updating Dependencies
```bash
# Update Rust dependencies
cd src-tauri
cargo update

# Update Node dependencies
npm update
```

## Known Issues

- On some Windows ARM64 systems, the `npm run tauri build` command may encounter file watcher issues. The development server (`npm run tauri dev`) works correctly.
- The application requires an active internet connection for federal rate calculations.

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Federal interest rate data provided by [Federal Reserve Economic Data (FRED)](https://fred.stlouisfed.org/)
- Built with [Tauri](https://tauri.app/), [React](https://react.dev/), and [Rust](https://www.rust-lang.org/)
- State rate information should be verified with official sources

## Support

For issues, questions, or suggestions, please [open an issue](https://github.com/matt793/JudgmentRateDesktop/issues) on GitHub.
