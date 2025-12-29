/**
 * Galactus HTTP API Client Example - JavaScript/Node.js
 * -------------------------------------------------------
 * Demonstrates integration with Galactus intent inference API using fetch.
 * 
 * Requirements:
 *   npm install node-fetch
 * 
 * Usage:
 *   node javascript-fetch-client.js
 *   GALACTUS_API_URL=http://localhost:8080 node javascript-fetch-client.js
 */

const fetch = require('node-fetch');

/**
 * Galactus API Client
 */
class GalactusClient {
    /**
     * Initialize Galactus API client
     * @param {string} baseUrl - API base URL
     * @param {string} apiKey - API authentication key
     * @param {number} timeout - Request timeout in milliseconds
     */
    constructor(baseUrl, apiKey, timeout = 10000) {
        this.baseUrl = baseUrl.replace(/\/$/, '');
        this.apiKey = apiKey;
        this.timeout = timeout;
        
        // Default headers
        this.headers = {
            'X-API-Key': this.apiKey,
            'Content-Type': 'application/json',
            'User-Agent': 'galactus-javascript-client/0.1.0'
        };
    }

    /**
     * Make HTTP request with timeout
     * @private
     */
    async _makeRequest(url, options) {
        const controller = new AbortController();
        const timeoutId = setTimeout(() => controller.abort(), this.timeout);

        try {
            const response = await fetch(url, {
                ...options,
                signal: controller.signal,
                headers: {
                    ...this.headers,
                    ...options.headers
                }
            });

            clearTimeout(timeoutId);
            return response;
        } catch (error) {
            clearTimeout(timeoutId);
            if (error.name === 'AbortError') {
                throw new Error('Request timeout');
            }
            throw error;
        }
    }

    /**
     * Compute single intent vector from signals
     * @param {Array} signals - List of signal inputs
     * @param {string} clientId - Client identifier
     * @param {Object} context - Additional context data
     * @param {Object} metadata - Request metadata
     * @returns {Promise<Object>} Intent computation result
     */
    async computeIntent(signals, clientId = 'javascript_client', context = {}, metadata = {}) {
        const url = `${this.baseUrl}/api/v1/intent`;
        
        const payload = {
            signals,
            client_id: clientId,
            context,
            metadata
        };

        console.debug(`Sending request to ${url}`);
        const startTime = Date.now();

        try {
            const response = await this._makeRequest(url, {
                method: 'POST',
                body: JSON.stringify(payload)
            });

            const elapsedMs = Date.now() - startTime;

            if (!response.ok) {
                const errorText = await response.text();
                let errorData;
                try {
                    errorData = JSON.parse(errorText);
                } catch {
                    errorData = { error: { code: 'UNKNOWN_ERROR', message: errorText } };
                }
                throw new APIError(response.status, errorData.error);
            }

            console.info(`Intent computed successfully (took ${elapsedMs}ms)`);
            return await response.json();

        } catch (error) {
            if (error instanceof APIError) {
                throw error;
            }
            console.error(`Request failed: ${error.message}`);
            throw error;
        }
    }

    /**
     * Compute multiple intent vectors in batch
     * @param {Array} requestsList - List of intent requests
     * @param {Object} metadata - Batch metadata
     * @returns {Promise<Object>} Batch computation results
     */
    async computeBatch(requestsList, metadata = {}) {
        const url = `${this.baseUrl}/api/v1/intent/batch`;
        
        const payload = {
            requests: requestsList,
            metadata
        };

        console.debug(`Sending batch request with ${requestsList.length} items`);
        const startTime = Date.now();

        try {
            const response = await this._makeRequest(url, {
                method: 'POST',
                body: JSON.stringify(payload)
            });

            const elapsedMs = Date.now() - startTime;

            if (!response.ok) {
                const errorData = await response.json();
                throw new APIError(response.status, errorData.error);
            }

            console.info(`Batch computed successfully (took ${elapsedMs}ms)`);
            return await response.json();

        } catch (error) {
            if (error instanceof APIError) {
                throw error;
            }
            console.error(`Batch request failed: ${error.message}`);
            throw error;
        }
    }

    /**
     * Check API service health
     * @returns {Promise<Object>} Health status information
     */
    async healthCheck() {
        const url = `${this.baseUrl}/api/v1/health`;

        try {
            const response = await this._makeRequest(url, {
                method: 'GET'
            });

            if (!response.ok) {
                throw new Error(`Health check failed: ${response.status}`);
            }

            return await response.json();
        } catch (error) {
            console.error(`Health check failed: ${error.message}`);
            throw error;
        }
    }

    /**
     * Get API service metrics
     * @returns {Promise<Object>} Service metrics
     */
    async getMetrics() {
        const url = `${this.baseUrl}/api/v1/metrics`;

        try {
            const response = await this._makeRequest(url, {
                method: 'GET'
            });

            if (!response.ok) {
                throw new Error(`Metrics request failed: ${response.status}`);
            }

            return await response.json();
        } catch (error) {
            console.error(`Metrics request failed: ${error.message}`);
            throw error;
        }
    }
}

/**
 * Custom API Error class
 */
class APIError extends Error {
    constructor(statusCode, errorData) {
        super(errorData.message);
        this.name = 'APIError';
        this.statusCode = statusCode;
        this.code = errorData.code;
        this.details = errorData.details || {};
    }
}

/**
 * Example: Compute single intent vector
 */
async function exampleSingleIntent() {
    console.log('\n=== Example: Single Intent Computation ===\n');

    // Get configuration from environment
    const apiUrl = process.env.GALACTUS_API_URL || 'http://localhost:8080';
    const apiKey = process.env.GALACTUS_API_KEY || 'test-api-key';

    const client = new GalactusClient(apiUrl, apiKey);

    try {
        // Check service health
        const health = await client.healthCheck();
        console.log(`Service status: ${health.status}`);
        console.log(`Service version: ${health.version}`);
    } catch (error) {
        console.warn('Warning: Health check failed, continuing anyway...');
    }

    // Prepare signal inputs
    const signals = [
        {
            name: 'oi_decay',
            value: -0.4,
            confidence: 0.85,
            timestamp: Math.floor(Date.now() / 1000),
            metadata: {}
        }
    ];

    try {
        // Compute intent
        const result = await client.computeIntent(signals, 'example_client');

        // Extract intent vector
        const intent = result.result.intent;
        console.log('\nIntent Vector:');
        console.log(`  Pressure: ${intent.pressure.toFixed(4)}`);
        console.log(`  Confidence: ${intent.confidence.toFixed(4)}`);
        console.log(`  Regime: ${intent.regime || 'unknown'}`);
        console.log(`  Timestamp: ${intent.timestamp}`);

        // Show signal contributions
        console.log('\nSignal Contributions:');
        for (const [signalName, contribution] of Object.entries(intent.signals || {})) {
            console.log(`  ${signalName}:`);
            console.log(`    Value: ${contribution.value.toFixed(4)}`);
            console.log(`    Weight: ${contribution.weight.toFixed(4)}`);
            console.log(`    Confidence: ${contribution.confidence.toFixed(4)}`);
        }

    } catch (error) {
        if (error instanceof APIError) {
            console.error(`API Error ${error.statusCode}: ${error.code} - ${error.message}`);
            console.error('Details:', error.details);
        } else {
            console.error(`Error computing intent: ${error.message}`);
        }
        process.exit(1);
    }
}

/**
 * Example: Batch intent computation
 */
async function exampleBatchComputation() {
    console.log('\n=== Example: Batch Intent Computation ===\n');

    const apiUrl = process.env.GALACTUS_API_URL || 'http://localhost:8080';
    const apiKey = process.env.GALACTUS_API_KEY || 'test-api-key';

    const client = new GalactusClient(apiUrl, apiKey);

    // Prepare multiple requests
    const requestsList = [];
    for (let i = 0; i < 3; i++) {
        requestsList.push({
            signals: [
                {
                    name: 'oi_decay',
                    value: -0.3 - (i * 0.1),
                    confidence: 0.8 + (i * 0.05),
                    timestamp: Math.floor(Date.now() / 1000),
                    metadata: {}
                }
            ],
            client_id: `batch_client_${i}`
        });
    }

    try {
        const result = await client.computeBatch(requestsList);

        console.log(`Batch computation completed:`);
        console.log(`  Requests processed: ${result.responses.length}`);

        result.responses.forEach((response, idx) => {
            const intent = response.result.intent;
            console.log(`\n  Request ${idx + 1}:`);
            console.log(`    Pressure: ${intent.pressure.toFixed(4)}`);
            console.log(`    Confidence: ${intent.confidence.toFixed(4)}`);
        });

    } catch (error) {
        if (error instanceof APIError) {
            console.error(`API Error ${error.statusCode}: ${error.code} - ${error.message}`);
        } else {
            console.error(`Error in batch computation: ${error.message}`);
        }
        process.exit(1);
    }
}

/**
 * Example: Error handling
 */
async function exampleErrorHandling() {
    console.log('\n=== Example: Error Handling ===\n');

    const apiUrl = process.env.GALACTUS_API_URL || 'http://localhost:8080';
    const apiKey = process.env.GALACTUS_API_KEY || 'test-api-key';

    const client = new GalactusClient(apiUrl, apiKey);

    // Try invalid signal value
    const invalidSignals = [
        {
            name: 'test_signal',
            value: 1.5,  // Invalid: outside [-1.0, 1.0] range
            confidence: 0.85,
            timestamp: Math.floor(Date.now() / 1000),
            metadata: {}
        }
    ];

    try {
        await client.computeIntent(invalidSignals);
        console.log('Unexpected success with invalid data');
    } catch (error) {
        if (error instanceof APIError) {
            console.log('✓ Validation error caught correctly:');
            console.log(`  Status: ${error.statusCode}`);
            console.log(`  Code: ${error.code}`);
            console.log(`  Message: ${error.message}`);
        } else {
            console.error(`Unexpected error type: ${error.message}`);
        }
    }
}

/**
 * Main entry point
 */
async function main() {
    console.log('='.repeat(60));
    console.log('Galactus API Client Examples (JavaScript)');
    console.log('='.repeat(60));

    try {
        // Run examples
        await exampleSingleIntent();
        await exampleBatchComputation();
        await exampleErrorHandling();

        console.log('\n' + '='.repeat(60));
        console.log('All examples completed successfully!');
        console.log('='.repeat(60) + '\n');

    } catch (error) {
        console.error('\nUnexpected error:', error);
        process.exit(1);
    }
}

// Run if executed directly
if (require.main === module) {
    main();
}

// Export for use as library
module.exports = {
    GalactusClient,
    APIError
};
