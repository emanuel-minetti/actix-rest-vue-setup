// https://on.cypress.io/api

describe('Visiting the home page', () => {
  it('loads', () => {
    cy.visit('');
    cy.get('.navbar > .container-fluid > :nth-child(1) > .h1').contains('Vite App');
    cy.get('main > .h1.text-center').contains('Home');
  });

  it('loads about lazy', () => {
    let counter = 0;
    cy.intercept('assets/AboutView**', () => {
      counter++;
    });

    cy.visit('');
    expect(counter).to.equal(0);
    cy.get('.navbar-nav > :nth-child(2) > .nav-link')
      .contains('About')
      .click()
      .then(() => {
        expect(counter).to.equal(1);
      });
  });

  it('loads imprint lazy', () => {
    let counter = 0;
    cy.intercept('assets/ImprintView**', () => {
      counter++;
    });

    cy.visit('');
    expect(counter).to.equal(0);
    cy.get('footer ul > li')
      .contains('Imprint')
      .click()
      .then(() => {
        expect(counter).to.equal(1);
      });
  });
});
