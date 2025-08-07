document.addEventListener('DOMContentLoaded', () => {
    // --- Element References ---
    const elements = {
        lottoTypeSelect: document.getElementById('lotto-type-select'),
        scrapeBtn: document.getElementById('scrape-btn'),
        progressContainer: document.getElementById('progress-container'),
        tableContainer: document.getElementById('table-container'),
        resultsHead: document.getElementById('results-head'),
        resultsBody: document.getElementById('results-body'),
        analysisSection: document.getElementById('analysis-section'),
        predictionTypeGroup: document.getElementById('prediction-type-group'),
        numberInput: document.getElementById('number-input'),
        analysisMethodSelect: document.getElementById('analysis-method-select'),
        analyzeBtn: document.getElementById('analyze-btn'),
        analysisResultsContainer: document.getElementById('analysis-results-container'),
    };

    // --- Application State ---
    const state = {
        scrapedResultsData: [],
        scrapeStatusInterval: null,
        // Default to 'thai', will be updated on change.
        currentLottoType: 'thai',
    };

    // --- Configuration for Different Lottery Types ---
    const LOTTO_CONFIG = {
        thai: {
            headers: ['วันที่ออกรางวัล', 'รางวัลที่ 1', 'เลขท้าย 2 ตัว'],
            prizes: {
                'prize1': { name: 'รางวัลที่ 1', key: 'Prize 1' },
                'prize2': { name: 'เลขท้าย 2 ตัว', key: 'Prize 2' },
            }
        },
        laos: {
            headers: ['วันที่ออกรางวัล', 'รางวัล 3 ตัว', 'รางวัล 2 ตัว'],
            prizes: {
                'prize1': { name: 'รางวัล 3 ตัว', key: 'Prize 1' },
                'prize2': { name: 'รางวัล 2 ตัว', key: 'Prize 2' },
            }
        }
    };

    // --- Event Listeners ---
    elements.scrapeBtn.addEventListener('click', handleScrapeButtonClick);
    elements.analyzeBtn.addEventListener('click', handleAnalyzeButtonClick);
    elements.lottoTypeSelect.addEventListener('change', (e) => {
        state.currentLottoType = e.target.value;
        // Reset UI when switching types
        resetUIForNewType();
    });

    // --- Functions ---

    function resetUIForNewType() {
        state.scrapedResultsData = [];
        elements.tableContainer.style.display = 'none';
        elements.analysisSection.style.display = 'none';
        elements.resultsBody.innerHTML = '';
        elements.resultsHead.innerHTML = '';
        elements.progressContainer.innerHTML = '';
        elements.progressContainer.style.display = 'none';
        elements.analysisResultsContainer.innerHTML = '';
    }

    async function handleScrapeButtonClick() {
        setScraperUIState(true);
        try {
            const response = await fetch('/start-scrape', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ lotto_type: state.currentLottoType })
            });

            if (!response.ok) {
                const errData = await response.json();
                throw new Error(errData.error || `ข้อผิดพลาดจากเซิร์ฟเวอร์: ${response.status}`);
            }
            // Start polling for status updates.
            state.scrapeStatusInterval = setInterval(checkScrapeStatus, 2000);
        } catch (error) {
            showError(elements.progressContainer, error.message);
            setScraperUIState(false);
        }
    }

    async function checkScrapeStatus() {
        try {
            const response = await fetch('/status');
            const data = await response.json();
            
            // Display progress logs.
            elements.progressContainer.innerHTML = data.progress.join('<br>');
            elements.progressContainer.scrollTop = elements.progressContainer.scrollHeight;

            // If the task is no longer running, process the results.
            if (!data.is_running) {
                clearInterval(state.scrapeStatusInterval);
                state.scrapedResultsData = data.results;
                displayScrapeResults(data.results);
                setScraperUIState(false);
            }
        } catch (error) {
            showError(elements.progressContainer, `เกิดข้อผิดพลาดในการตรวจสอบสถานะ: ${error.message}`);
            clearInterval(state.scrapeStatusInterval);
            setScraperUIState(false);
        }
    }

    function displayScrapeResults(results) {
        if (!results || results.length === 0) {
            elements.progressContainer.innerHTML += '<br><br><strong>ไม่พบข้อมูลจากการดึงข้อมูล</strong>';
            return;
        }

        const config = LOTTO_CONFIG[state.currentLottoType];
        // Update Table Headers
        elements.resultsHead.innerHTML = `<tr>${config.headers.map(h => `<th>${h}</th>`).join('')}</tr>`;

        // Update Table Body
        elements.resultsBody.innerHTML = results.map(result => `
            <tr>
                <td>${result['Draw Date']}</td>
                <td><strong>${result['Prize 1']}</strong></td>
                <td><strong>${result['Prize 2'] || ''}</strong></td>
            </tr>
        `).join('');

        // Show table and analysis section
        elements.tableContainer.style.display = 'block';
        elements.analysisSection.style.display = 'block';

        // Update the analysis UI based on the new data
        updateAnalysisUI();
    }
    
    function updateAnalysisUI() {
        const config = LOTTO_CONFIG[state.currentLottoType];
        const prizeKeys = Object.keys(config.prizes);

        // Update Prediction Target Radio Buttons
        elements.predictionTypeGroup.innerHTML = prizeKeys.map((key, index) => `
            <input type="radio" id="predict-${key}" name="prediction_type" value="${key}" ${index === 0 ? 'checked' : ''}>
            <label for="predict-${key}">${config.prizes[key].name}</label>
        `).join('');
        
        // Add event listeners to the newly created radio buttons
        document.querySelectorAll('input[name="prediction_type"]').forEach(radio => {
            radio.addEventListener('change', (e) => updateAnalysisInput(e.target.value));
        });

        // Trigger the change event to populate the textarea with the default prize type
        updateAnalysisInput(prizeKeys[0]);
    }

    function updateAnalysisInput(prizeId) {
        if (state.scrapedResultsData.length === 0) return;
        
        // Get the data key (e.g., 'Prize 1') from the config using the prizeId (e.g., 'prize1')
        const dataKey = LOTTO_CONFIG[state.currentLottoType].prizes[prizeId].key;
        
        // Map the results to get the correct prize numbers, reverse for chronological order, and filter out any empty values.
        const numbersForAnalysis = [...state.scrapedResultsData].reverse()
            .map(result => result[dataKey]?.replace(/[^0-9]/g, ''))
            .filter(Boolean);
            
        elements.numberInput.value = numbersForAnalysis.join(', ');
        // Clear previous analysis results when the input changes
        elements.analysisResultsContainer.innerHTML = '';
    }

    async function handleAnalyzeButtonClick() {
        const numbersText = elements.numberInput.value;
        if (!numbersText.trim()) {
            showError(elements.analysisResultsContainer, 'กรุณาใส่ชุดตัวเลขเพื่อวิเคราะห์');
            return;
        }
        const numbersArray = numbersText.split(',').map(s => s.trim()).filter(s => s);
        
        setAnalyzerUIState(true);
        try {
            const response = await fetch('/analyze', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ 
                    numbers: numbersArray, 
                    method: elements.analysisMethodSelect.value 
                })
            });
            const resultData = await response.json();
            if (!response.ok || resultData.error) {
                throw new Error(resultData.error || 'การวิเคราะห์ล้มเหลว');
            }
            displayAnalysisResults(resultData);
        } catch (error) {
            showError(elements.analysisResultsContainer, `ข้อผิดพลาดในการวิเคราะห์: ${error.message}`);
        } finally {
            setAnalyzerUIState(false, true); // Keep results visible after analysis
        }
    }

    function displayAnalysisResults(data) {
        const { statistical_summary, pattern_analysis, prediction_output, detailed_explanation } = data;
        
        // Helper function to create a list block, now handles both objects and arrays
        const createListHtml = (title, dataObj) => {
            if (!dataObj || Object.keys(dataObj).length === 0) return '';
            
            let items;
            if (Array.isArray(dataObj)) {
                items = dataObj.map(value => `<li>${value}</li>`).join('');
            } else {
                items = Object.entries(dataObj)
                    .map(([key, value]) => `<li><strong>${key}:</strong> ${Array.isArray(value) ? value.join(', ') : JSON.stringify(value).replace(/"/g, '')}</li>`)
                    .join('');
            }
            return `<div class="result-block"><h3>${title}</h3><ul>${items}</ul></div>`;
        };
        
        const createParagraphHtml = (title, dataObj) => {
            if (!dataObj || Object.keys(dataObj).length === 0) return '';
             const items = Object.entries(dataObj)
                .map(([key, value]) => `<h4>${key}</h4><p>${value}</p>`)
                .join('');
            return `<div class="result-block"><h3>${title}</h3>${items}</div>`;
        };

        const predictionHtml = `
            <div class="result-block prediction-block">
                <h3>🔮 คำทำนายจาก AI</h3>
                <div class="prediction-value">${prediction_output.PREDICTION}</div>
                <div class="confidence"><strong>โมเดล:</strong> ${prediction_output.METHOD || 'N/A'}</div>
            </div>`;

        // Generate HTML for alternative predictions if they exist
        const alternativesHtml = prediction_output['ทางเลือกอื่นๆ'] ? 
            createListHtml('🎲 ทางเลือกอื่นๆ', prediction_output['ทางเลือกอื่นๆ']) : '';

        elements.analysisResultsContainer.innerHTML = [
            predictionHtml,
            alternativesHtml, // Add alternatives right after the main prediction
            createListHtml('📊 สรุปสถิติ', statistical_summary),
            createListHtml('🧩 การวิเคราะห์รูปแบบ', pattern_analysis),
            createParagraphHtml('📝 คำอธิบายโดยละเอียด', detailed_explanation)
        ].join('');
    }
    
    // --- UI State Management ---
    function setScraperUIState(isScraping) {
        elements.scrapeBtn.disabled = isScraping;
        elements.lottoTypeSelect.disabled = isScraping;
        elements.progressContainer.style.display = isScraping ? 'block' : 'none';
        if (isScraping) {
            resetUIForNewType();
            elements.progressContainer.style.display = 'block';
        }
    }

    function setAnalyzerUIState(isAnalyzing, keepContent = false) {
        elements.analyzeBtn.disabled = isAnalyzing;
        if (isAnalyzing) {
            elements.analysisResultsContainer.innerHTML = `
                <div class="spinner-container">
                    <div class="spinner"></div>
                    <p>AI กำลังประมวลผลข้อมูล...</p>
                </div>`;
        } else if (!keepContent) {
            elements.analysisResultsContainer.innerHTML = '';
        }
    }

    function showError(container, message) {
        container.innerHTML = `<p class="error">${message}</p>`;
        container.style.display = 'block';
    }
});