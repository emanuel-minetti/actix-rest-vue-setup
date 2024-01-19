import type { LocaleAbbrev } from '../../src/i18n/Translations';

describe('Visiting the app', () => {
  const DEFAULT_LANG: LocaleAbbrev = 'de';
  const FLAG_ABBREV = DEFAULT_LANG == 'en' ? 'gb' : DEFAULT_LANG;

  beforeEach(() => {
    cy.intercept('/api/config', () => {}).as('getConfig');
  });

  it('shows a flag for the default language', () => {
    cy.visit('')
      .wait('@getConfig')
      .get('.fi')
      .should('have.class', 'fi-' + FLAG_ABBREV);
  });

  it('shows a select for language switching after clicking the flag', () => {
    cy.visit('')
      .wait('@getConfig')
      .get('#ls-select')
      .should('not.exist')
      .get('.fi')
      .should('have.class', 'fi-' + FLAG_ABBREV)
      .click()
      .get('#ls-select')
      .should('exist')
      .get('body')
      .click()
      .get('#ls-select')
      .should('not.exist');
  });

  it('shows a select for language switching after pressing ENTER on the flag', () => {
    cy.visit('')
      .wait('@getConfig')
      .get('#ls-select')
      .should('not.exist')
      .get('.fi')
      .should('have.class', 'fi-' + FLAG_ABBREV)
      .type('{enter}')
      .get('#ls-select')
      .should('exist')
      .get('body')
      .click()
      .get('#ls-select')
      .should('not.exist');
  });

  it('closes select for language switching after clicking the same language', () => {
    cy.visit('')
      .wait('@getConfig')
      .get('#ls-select')
      .should('not.exist')
      .get('.fi')
      .should('have.class', 'fi-' + FLAG_ABBREV)
      .click()
      .get('#ls-select')
      .should('exist')
      .select(DEFAULT_LANG)
      .get('#ls-select')
      .should('not.exist');
  });

  it('changes the flag and closes select for language switching after clicking another language', () => {
    cy.visit('')
      .wait('@getConfig')
      .get('#ls-select')
      .should('not.exist')
      .get('.fi')
      .should('have.class', 'fi-' + FLAG_ABBREV)
      .click()
      .get('#ls-select')
      .should('exist')
      .children()
      .not('[value][value="' + DEFAULT_LANG + '"]:first')
      .click({ force: true })
      .get('.fi')
      .should('not.have.class', 'fi-' + FLAG_ABBREV)
      .get('#ls-select')
      .should('not.exist');
  });

  it('closes the select for language switching after pressing ENTER on the element', () => {
    cy.visit('')
      .wait('@getConfig')
      .get('#ls-select')
      .should('not.exist')
      .get('.fi')
      .should('have.class', 'fi-' + FLAG_ABBREV)
      .type('{enter}')
      .get('#ls-select')
      .should('exist')
      .type('{enter}')
      .get('#ls-select')
      .should('not.exist');
  });
});
