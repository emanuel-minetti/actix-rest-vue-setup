describe('Visiting the app', () => {
  it('shows copyright and version information', () => {
    let counter = 0;
    cy.intercept('/api/config', (req) => {
      counter++;
      // req.on('response', (res) => {
      //   res.headers = {
      //     'Content-Type': 'application/json',
      //   };
      //   res.body = JSON.stringify({
      //     copyright: '© Example.com 2023-24',
      //     version: '0.0.1',
      //     global_message: '',
      //   });
      // });
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
      .then((el) => {
        let text = el.text();
        text = text.substring(7);
        expect(text).not.to.equal('function String() { [native code] }');
      });
  });
});
