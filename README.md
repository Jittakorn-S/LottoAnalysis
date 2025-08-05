# 🤖 AI Lottery Analysis (Thai & Laos)

An intelligent web application that scrapes historical lottery data for both Thai and Laos lotteries. It uses various analytical models—including statistical analysis, numerology, machine learning, and Markov chains—to generate predictions.

[![Deploy to Render](https://render.com/images/deploy-to-render-button.svg)](https://render.com)

## ✨ Key Features

* **Multi-Country Support**: Scrapes and analyzes data for both **Thai (Sanook)** and **Laos (Expserve)** lotteries.
* **Multi-Model Analysis**: Choose from several AI and statistical models for prediction:
    * **Comprehensive Statistics**: A robust model based on frequency analysis to find the mode (most common number).
    * **Numerology**: Analyzes patterns using the ancient practice of digital roots.
    * **Machine Learning**: A Decision Tree model that learns from past results to predict the last digit of the next number.
    * **Markov Chains**: A probabilistic model that analyzes the transitional probability between lottery numbers.
* **Automated Data Scraping**: Fetches up-to-date lottery results directly from the web in the background.
* **Modern Web Interface**: A clean and responsive UI built with vanilla JavaScript, HTML, and CSS, allowing for dynamic content updates.
* **High-Performance Backend**: Built with Rust and the Actix Web framework for a fast, reliable, and concurrent experience. It also uses `mimalloc` for optimized memory allocation.
* **Containerized & Deployable**: Fully containerized with Docker for easy, consistent deployment anywhere. Includes a `render.yaml` for one-click deployment on [Render](https://render.com/).

## 🛠️ Technology Stack

* **Backend**: Rust, Actix Web, Tokio
* **Frontend**: Vanilla JavaScript, HTML5, CSS3
* **Data Scraping**: `reqwest`, `scraper`
* **AI/ML**: `linfa`, `ndarray`, `statrs`
* **Deployment**: Docker, Render

## 🚀 Getting Started

### Prerequisites

* **Rust Toolchain**: [Install from rust-lang.org](https://www.rust-lang.org/tools/install)
* **Docker**: (Optional, for containerized deployment) [Install from docker.com](https://www.docker.com/get-started)

### 1. Local Development

This is the fastest way to get the project running for development and testing.

```bash
# Clone the repository
git clone [https://github.com/Jittakorn-S/LottoAnalysis.git](https://github.com/Jittakorn-S/LottoAnalysis.git)
cd LottoAnalysis

# Run the application in release mode for best performance
cargo run --release

# Open your browser and navigate to [http://127.0.0.1:8080](http://127.0.0.1:8080)
