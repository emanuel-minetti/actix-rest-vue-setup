import { defineConfig } from 'cypress'

// noinspection JSUnusedGlobalSymbols
export default defineConfig({
  e2e: {
    specPattern: 'cypress/e2e/**/*.cy.ts',
    baseUrl: 'http://localhost:8080'
  }
})
