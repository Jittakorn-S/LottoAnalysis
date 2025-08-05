🤖 AI Lottery Analysis & PredictionAn intelligent web application that scrapes historical lottery data for both Thai (สลากไทย) and Laos (หวยลาว) lotteries. It leverages various analytical models—including statistical analysis, numerology, machine learning, and Markov chains—to generate insightful predictions and statistical summaries.The application is built with a high-performance Rust backend using the Actix web framework and a dynamic, vanilla JavaScript frontend, all containerized with Docker for easy deployment.➡️ Live Demo (Deployed on Render)<!-- Replace with an actual screenshot URL -->✨ Key Features🌍 Multi-Country Support: Scrapes and analyzes data for both Thai and Laos lotteries.🧠 Multiple AI Models: Choose from several sophisticated analysis methods:Comprehensive Statistics: A robust model based on frequency, mean, median, and mode.Numerology: Analyzes patterns using the ancient practice of digital roots.Machine Learning (Decision Tree): A model that learns from historical data to predict the next number's final digit.Markov Chains: A probabilistic model that analyzes the statistical likelihood of transitioning from one number to the next.☁️ Automated Data Scraping: Fetches up-to-date lottery results directly from the web in a non-blocking background process.🖥️ Modern Web Interface: A clean, responsive, and intuitive UI built with vanilla JavaScript, HTML, and CSS. The interface updates dynamically without page reloads.🚀 High-Performance Backend: Built with Rust and Actix Web for a fast, memory-safe, and concurrent experience, capable of handling analysis without blocking the server.🐳 Containerized & Deployable: Fully containerized with Docker for easy, consistent deployment. Includes a render.yaml for one-click deployment on Render.🛠️ Technology StackBackend:Language: RustWeb Framework: Actix WebAsync Runtime: TokioWeb Scraping: Reqwest & ScraperMachine Learning: linfaFrontend:HTML5CSS3 (Modern Design)Vanilla JavaScript (ES6+)Deployment:DockerRender🚀 Getting StartedFollow these instructions to get the project running on your local machine for development and testing purposes.PrerequisitesRust Toolchain: Install from rust-lang.org. Ensure you have version 1.79 or newer.Docker: Install from docker.com.1. Local Development (with Cargo)This is the fastest way to get the project running for development.# Clone the repository
git clone https://github.com/Jittakorn-S/LottoAnalysis.git
cd LottoAnalysis

# Run the application in release mode for the best performance
cargo run --release

# Open your browser and navigate to http://127.0.0.1:8080
2. Local Development (with Docker)This method builds and runs the application inside a Docker container, mirroring the production environment.# Clone the repository
git clone https://github.com/Jittakorn-S/LottoAnalysis.git
cd LottoAnalysis

# Build the Docker image
docker build -t lotto-analysis .

# Run the Docker container
docker run -p 8080:8080 -d --name lotto-app lotto-analysis

# Open your browser and navigate to http://127.0.0.1:8080
☁️ Deployment to RenderThis project is configured for seamless deployment to Render using the render.yaml file.Fork this repository on GitHub.Go to the Render Dashboard and click New + > Blueprint.Connect the GitHub repository you just forked.Render will automatically detect the render.yaml file and configure the service.Click Approve, and your application will be deployed. The first build may take a few minutes.Render will automatically redeploy your application whenever you push changes to your repository's main branch.📁 Project StructureThe project is organized into a standard Rust binary crate structure with modules for clear separation of concerns.LottoAnalysis/
├── .gitignore         # Specifies files to be ignored by Git
├── Cargo.lock         # Exact versions of dependencies
├── Cargo.toml         # Project metadata and dependencies
├── Dockerfile         # Instructions for building the Docker image
├── README.md          # This file
├── render.yaml        # Configuration for Render deployment
├── src/               # Source code directory
│   ├── analysis.rs    # All analysis and prediction logic
│   ├── main.rs        # Main application entry point and API routes
│   ├── models.rs      # Data structures (structs, enums)
│   └── scraper.rs     # Web scraping logic
├── static/            # Frontend static assets
│   ├── app.js         # Main JavaScript file for the UI
│   └── styles.css     # CSS for styling the application
└── templates/         # HTML templates
    └── index.html     # The main HTML file for the single-page app
💡 How It WorksData Scraping: When a user initiates a scrape, the Actix backend spawns a non-blocking tokio task. This task uses reqwest to fetch HTML from the target lottery websites and scraper to parse the results. The progress is reported back to the frontend, and the final results are stored in shared application state.Analysis: The user selects an analysis model and a set of numbers. The frontend sends this data to the /analyze endpoint. The backend uses web::block to run the CPU-intensive analysis in a separate thread pool, preventing the server from becoming unresponsive. The chosen analysis function from the analysis module processes the data and returns a structured JSON response.Frontend: The vanilla JavaScript frontend handles all user interactions, API calls, and dynamic DOM updates. It polls the /status endpoint for scraping progress and renders all results and analysis without requiring a page refresh.🤝 ContributingContributions are welcome! If you have ideas for new features, improvements, or bug fixes, please feel free to:Fork the repository.Create a new feature branch (git checkout -b feature/your-feature-name).Commit your changes (git commit -m 'Add some feature').Push to the branch (git push origin feature/your-feature-name).Open a Pull Request.📜 LicenseThis project is licensed under the MIT License. See the LICENSE file for details.