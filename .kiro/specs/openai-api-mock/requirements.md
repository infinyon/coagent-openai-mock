# Requirements Document

## Introduction

This feature involves creating a Rust crate that provides a mock HTTP server implementation of the OpenAI API. The mock server will conform to the official OpenAI OpenAPI specification and initially support completion and embedding endpoints. The server will enforce API key authentication and return syntactically valid fake responses that can be consumed by standard OpenAI clients. The implementation will be structured to allow easy extension to other API endpoints in the future.

## Requirements

### Requirement 1

**User Story:** As a developer testing OpenAI integrations, I want a mock server that implements the completion API endpoint, so that I can test my application without making real API calls or incurring costs.

#### Acceptance Criteria

1. WHEN a POST request is made to `/v1/completions` with valid authentication THEN the system SHALL return a syntactically valid completion response
2. WHEN the request includes model, prompt, and other standard parameters THEN the system SHALL accept them and include appropriate values in the response
3. WHEN the response is returned THEN it SHALL conform to the OpenAI API specification format
4. WHEN the fake response is consumed by a standard OpenAI client THEN it SHALL be accepted without errors

### Requirement 2

**User Story:** As a developer testing OpenAI integrations, I want a mock server that implements the embedding API endpoint, so that I can test embedding functionality without real API calls.

#### Acceptance Criteria

1. WHEN a POST request is made to `/v1/embeddings` with valid authentication THEN the system SHALL return a syntactically valid embedding response
2. WHEN the request includes input text and model parameters THEN the system SHALL accept them and return appropriate fake embedding vectors
3. WHEN the embedding response is returned THEN it SHALL include properly formatted vector arrays
4. WHEN the fake embedding response is consumed by a standard OpenAI client THEN it SHALL be accepted without errors

### Requirement 3

**User Story:** As a developer using the mock server, I want API key authentication to be enforced, so that the mock behaves like the real OpenAI API and I can test authentication flows.

#### Acceptance Criteria

1. WHEN a request is made without an Authorization header THEN the system SHALL return a 401 Unauthorized response
2. WHEN a request is made with an invalid API key THEN the system SHALL return a 401 Unauthorized response
3. WHEN a request is made with the correct hardcoded API key THEN the system SHALL process the request normally
4. WHEN authentication fails THEN the error response SHALL match the format used by the real OpenAI API
5. WHEN the API key is provided in the Authorization header as "Bearer {key}" THEN the system SHALL validate it correctly

### Requirement 4

**User Story:** As a developer extending the mock server, I want the code to be structured modularly using the Poem web framework, so that I can easily add support for additional OpenAI API endpoints in the future.

#### Acceptance Criteria

1. WHEN the codebase is organized THEN it SHALL separate concerns into distinct modules (handlers, models, authentication, etc.)
2. WHEN new endpoints need to be added THEN the existing code structure SHALL support adding them without major refactoring
3. WHEN response models are defined THEN they SHALL be reusable across different endpoints
4. WHEN the HTTP server is configured THEN it SHALL use Poem's routing system that allows easy addition of new routes
5. WHEN the web server is implemented THEN it SHALL use the Poem framework for HTTP handling

### Requirement 5

**User Story:** As a developer using the mock server, I want comprehensive documentation, so that I know how to run and configure the server.

#### Acceptance Criteria

1. WHEN the README is provided THEN it SHALL document the hardcoded API key value
2. WHEN the README is provided THEN it SHALL include instructions for running the server
3. WHEN the README is provided THEN it SHALL list the supported endpoints
4. WHEN the README is provided THEN it SHALL include example usage with curl or similar tools

### Requirement 6

**User Story:** As a developer maintaining the mock server, I want end-to-end tests using Hurl, so that I can verify the server behaves correctly and catch regressions.

#### Acceptance Criteria

1. WHEN e2e tests are written THEN they SHALL use Hurl as the testing framework
2. WHEN tests are run THEN they SHALL verify successful completion API calls with valid authentication
3. WHEN tests are run THEN they SHALL verify successful embedding API calls with valid authentication
4. WHEN tests are run THEN they SHALL verify authentication failures return appropriate error responses
5. WHEN tests are run THEN they SHALL validate response formats match expected OpenAI API structures
6. WHEN the test suite is executed THEN it SHALL provide clear pass/fail results

### Requirement 7

**User Story:** As a developer integrating with the mock server, I want it to conform to the official OpenAI OpenAPI specification, so that existing OpenAI client libraries work without modification.

#### Acceptance Criteria

1. WHEN the server implementation is created THEN it SHALL conform to the OpenAI OpenAPI specification at https://raw.githubusercontent.com/openai/openai-openapi/refs/heads/manual_spec/openapi.yaml
2. WHEN request validation is performed THEN it SHALL accept the same parameters as the real OpenAI API
3. WHEN responses are generated THEN they SHALL include all required fields as specified in the OpenAPI schema
4. WHEN content types and headers are handled THEN they SHALL match the OpenAPI specification requirements