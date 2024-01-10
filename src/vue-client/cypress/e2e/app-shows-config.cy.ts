describe('Visiting the app', () => {
  it('shows copyright and version information', () => {
    let counter = 0;
    cy.intercept('/api/config', (req) => {
      counter++;
      req.on('response', (res) => {
        res.headers = {
          'Content-Type': 'application/json',
        };
        res.body = { copyright: '© Example.com 2023-24', version: '0.0.1', global_message: '' };
      });
    }).as('getConfig');

    cy.visit('')
      .wait('@getConfig')
      .then(() => {
        expect(counter).to.equal(1);
      })
      .get('.col-4 > .list-unstyled > :nth-child(1)')
      .should('not.be.empty')
      .then((el) => {
        let text = el.text();
        expect(text).not.to.equal('function String() { [native code] }');
      })
      .get('.col-4 > .list-unstyled > :nth-child(2)')
      .should('not.be.empty')
      .then((el) => {
        let text = el.text();
        text = text.substring(7);
        expect(text).not.to.equal('function String() { [native code] }');
      });
  });

  it('shows a global message if there is one', () => {
    cy.intercept('/api/config', (req) => {
      req.on('response', (res) => {
        res.headers = {
          'Content-Type': 'application/json',
        };
        res.body = {
          copyright: '© Example.com 2023-24',
          version: '0.0.1',
          global_message: 'Global message',
        };
      });
    }).as('getConfig');

    cy.visit('')
      .wait('@getConfig')
      .get('#app-global-message')
      .should('exist')
      .get('#app-error-message')
      .should('not.exist');
  });

  it('shows no global message if there is none', () => {
    cy.intercept('/api/config', (req) => {
      req.on('response', (res) => {
        res.headers = {
          'Content-Type': 'application/json',
        };
        res.body = {
          copyright: '© Example.com 2023-24',
          version: '0.0.1',
          global_message: '',
        };
      });
    }).as('getConfig');

    cy.visit('')
      .wait('@getConfig')
      .get('#app-global-message')
      .should('not.exist')
      .get('#app-error-message')
      .should('not.exist');
  });

  it('shows an error message if the API request fails', () => {
    cy.intercept('/api/config', (req) => {
      req.destroy();
    }).as('getConfigFail');

    cy.visit('')
      .wait('@getConfigFail')
      .get('#app-global-message')
      .should('not.exist')
      .get('#app-error-message')
      .should('exist');
  });
});
