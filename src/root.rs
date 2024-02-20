use leptos::*;
use leptos_router::*;

use crate::account_dialog::dialog::AccountDialogView;
use crate::accounts::page::{AccountSpotlightPage, AccountsPage};
use crate::blocks::page::{BlockSpotlight, LatestBlocksPage};
use crate::broadcast::page::{
    BroadcastDelegationPage, BroadcastFromLedgerPage, BroadcastTransactionPage,
};
use crate::common::components::DelegationTabbedPage;
use crate::footer::Footer;
use crate::header::navigation::Header;
use crate::next_stakes::page::NextStakesPage;
use crate::snarks::page::SnarksPage;
use crate::stakes::page::StakesPage;
use crate::summary::page::SummaryPage;
use crate::transactions::page::{TransactionSpotlightPage, TransactionsPage};
use crate::common::components::NullView;

#[component]
pub fn Root() -> impl IntoView {
    view! {
        <Router>
            <Header/>
            <main>
                <Routes>
                    <Route path="/summary" view=SummaryPage>
                        <Route path="accounts/:id" view=AccountDialogView/>
                        <Route path="/*any" view=NullView/>
                    </Route>
                    <Route path="/accounts" view=AccountsPage/>
                    <Route path="/accounts/:id" view=AccountSpotlightPage/>
                    <Route path="/blocks" view=LatestBlocksPage>
                        <Route path="accounts/:id" view=AccountDialogView/>
                        <Route path="/*any" view=NullView/>
                    </Route>
                    <Route path="/blocks/:id" view=BlockSpotlight/>
                    <Route path="/transactions" view=TransactionsPage/>
                    <Route path="/transactions/:id" view=TransactionSpotlightPage/>
                    <Route path="/snarks" view=SnarksPage/>
                    <Route path="/stakes" view=StakesPage/>
                    <Route path="/next-stakes" view=NextStakesPage/>
                    <Route path="/broadcast" view=DelegationTabbedPage>
                        <Route path="/transaction" view=BroadcastTransactionPage/>
                        <Route path="/delegation" view=BroadcastDelegationPage/>
                        <Route path="/ledger" view=BroadcastFromLedgerPage/>
                        <Route path="/*any" view=BroadcastTransactionPage/>
                    </Route>
                    <Route path="/*any" view=SummaryPage/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
