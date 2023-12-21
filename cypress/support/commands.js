// ***********************************************
// This example commands.js shows you how to
// create various custom commands and overwrite
// existing commands.
//
// For more comprehensive examples of custom
// commands please read more here:
// https://on.cypress.io/custom-commands
// ***********************************************
//
//
// -- This is a parent command --
// Cypress.Commands.add('login', (email, password) => { ... })
//
//
// -- This is a child command --
// Cypress.Commands.add('drag', { prevSubject: 'element'}, (subject, options) => { ... })
//
//
// -- This is a dual command --
// Cypress.Commands.add('dismiss', { prevSubject: 'optional'}, (subject, options) => { ... })
//
//
// -- This will overwrite an existing command --
// Cypress.Commands.overwrite('visit', (originalFn, url, options) => { ... })

Cypress.Commands.add('closeAccountDialog', () => {
  cy.get('dialog button#closedialog a').click();
  cy.get('dialog').should('not.exist');
});

Cypress.Commands.add('openAccountDialog', (linkSelector) => {
  cy.get(linkSelector).first().click();
  cy.get('dialog').should('be.visible');
  cy.get('dialog').contains('Account Overview').should('be.visible');
});

Cypress.Commands.add('accountDialogToAccount', () => {
  cy.get('dialog button#viewmore a').click();
  cy.get('dialog').should('not.exist');

  cy.url().should('contain', 'http://localhost:5274/accounts')
});

