describe('Visiting the app', () => {
  it('shows a flag for the default language', () => {
    cy.intercept('/api/config', () => {}).as('getConfig');
    cy.visit('').wait('@getConfig').get('.fi').should('have.class', 'fi-gb');
  });

  it('shows a select for language switching after clicking the flag', () => {
    cy.intercept('/api/config', () => {}).as('getConfig');
    cy.visit('')
      .wait('@getConfig')
      .get('#ls-select')
      .should('not.exist')
      .get('.fi')
      .should('have.class', 'fi-gb')
      .click()
      .get('#ls-select')
      .should('exist')
      .get('body')
      .click()
      .get('#ls-select')
      .should('not.exist');
  });
});
