suite(["@CI"],'snarks page', () => {
    [{
        origin: '/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6',
        dest:"snarks",
        href:"/snarks?account=B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6"
    }].forEach(({origin, dest, href}) => it(`is navigated to from ${dest}`,() => {
        cy.visit(origin);
        cy.get('a').contains("See all snark jobs", {timeout: 60000}).click();
        cy.url().should('contain', href);
    }));
});
