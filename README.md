# Low Latency Trading System

This project aims to develop a low-latency trading system capable of connecting to the Binance data source, fetching real-time market data, analyzing it, and executing trades based on predefined decision logic. The system is implemented in Rust to leverage its performance and safety features.

## Project Milestones

- [x] **Setup Development Environment**: Configure the development environment for Rust and ensure all necessary tools are installed.
- [x] **Setup Required Dependencies**: Install and manage all external libraries and crates required for the project.
- [x] **Connect to Binance Data Source**: Establish a connection to Binance's API to retrieve live market data.
- [x] **Setup API Fetching**: Implement functionality to fetch and handle data from the Binance API.
- [x] **Test 1**: Conduct initial tests to ensure data fetching and handling are working as expected.
- [x] **Store and Analyze Data**: Develop mechanisms to store the fetched data and perform analysis to derive meaningful insights.
- [x] **Add Simple Decision Logic**: Implement basic trading logic to make buy/sell decisions based on analyzed data.
- [x] **Logging Trades**: Create a logging system to record all executed trades for auditing and analysis purposes.
- [x] **Enhancing the Trade System**: Improve the trading system's performance and reliability.
- [x] **Adding Performance Metrics**: Introduce metrics to monitor and evaluate the system's performance.

## Getting Started

To get a local copy up and running, follow these steps:

### Prerequisites

- **Rust**: Ensure that Rust is installed on your system. You can download it from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/Harshcreator/low_latency_trading_system.git
   cd low_latency_trading_system
   ```
2. **Install Dependencies: Use Cargo, Rust's package manager, to install the required dependencies:**
   ```bash
   cargo build
   ```
3. **Set Up Environment Variables: Create a .env file in the root directory and add your Binance API credentials:**
  ```bash
  BINANCE_API_KEY=your_api_key
  BINANCE_SECRET_KEY=your_secret_key
  ```
4. **Run the Application: Start the application using Cargo:**
   ```bash
    cargo run 
   ```
## Usage

Once the application is running, it will:

1. **Connect to the Binance API**: The system will establish a connection to the Binance data source using your provided API credentials.
2. **Fetch Real-Time Market Data**: The system will continuously pull live market data from Binance, including pricing, volume, and other relevant metrics.
3. **Analyze the Data**: Based on the predefined decision logic and algorithms, the system will perform analysis on the data to identify potential trading opportunities.
4. **Execute Trades**: Once a trade decision is made, the system will automatically place buy or sell orders via the Binance API.
5. **Log Trades**: All executed trades will be logged for future reference, ensuring an audit trail of actions taken by the system.
    

