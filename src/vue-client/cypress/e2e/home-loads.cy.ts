// https://on.cypress.io/api

describe('Visiting the home page', () => {
  it('loads', () => {
    cy.visit('')
    cy.get('.navbar > .container-fluid > :nth-child(1) > .h1').contains('Vite App')
    cy.get('main > .data-arvs-top-title').contains('Home')
    // cy.contains('a', 'About').click()
    // cy.contains('h1', 'This is an about page')
  })

  it('loads about lazy', () => {
    let aboutCounter = 0

    cy.intercept('assets/AboutView**', () => {
      aboutCounter++
    })
    cy.visit('')
    expect(aboutCounter).to.equal(0)
    cy.get('.navbar-nav > :nth-child(2) > .nav-link').contains('About').click()
    cy.wait(1000).then(() => {
      expect(aboutCounter).to.equal(1)
    })
  })

  it('loads imprint lazy', () => {
    let imprintCounter = 0

    cy.intercept('assets/ImprintView**', () => {
      imprintCounter++
    })
    cy.visit('')
    expect(imprintCounter).to.equal(0)
    cy.get('.data-arvs-footer-links > li').contains('Imprint').click()
    cy.wait(3000).then(() => {
      expect(imprintCounter).to.equal(1)
    })
  })
})
