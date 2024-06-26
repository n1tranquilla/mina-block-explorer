suite(["@tier1"], "account spotlight", () => {
  let dialogs = [
    {
      origin:
        "/blocks/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6",
      selector: "#viewmore a",
    },
    {
      origin:
        "/blocks/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6",
      selector: "#viewmore a",
    },
  ];

  dialogs.forEach(({ origin, selector }) =>
    it(`is navigated to from ${origin}`, () => {
      cy.visit(origin);
      cy.wait(1000);
      cy.get(selector).first().click({ force: true });
      cy.wait(1000);
      cy.url().should("include", "/accounts/");
    }),
  );
});
