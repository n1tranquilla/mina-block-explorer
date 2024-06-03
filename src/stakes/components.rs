use super::{functions::*, models::*};
use crate::{
    common::{
        constants::*,
        functions::convert_to_link,
        models::{MyError, TableMetadata},
        table::*,
    },
    summary::functions::load_data as load_summary_data,
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn StakesPageContents() -> impl IntoView {
    let (metadata, set_metadata) = create_signal(Some(TableMetadata::default()));
    let (epoch_sig, _) = create_query_signal::<i64>("epoch");
    let query_params_map = use_query_map();

    let summary_resource = create_resource(|| (), |_| async move { load_summary_data().await });

    let current_epoch = move || match summary_resource.get() {
        Some(Ok(data)) => Some(data.epoch as i64),
        _ => None,
    };

    let (ledger_hash, set_ledger_hash) = create_signal(None::<String>);

    let resource = create_resource(
        move || (epoch_sig.get(), current_epoch(), query_params_map.get()),
        move |(epoch_opt, c_epoch, params_map)| async move {
            match (c_epoch, epoch_opt) {
                (Some(epoch), None) | (_, Some(epoch)) => {
                    let public_key = params_map.get("q-key").cloned();
                    let delegate = params_map.get("q-delegate").cloned();
                    let response = load_data(Some(epoch), public_key, delegate).await;
                    match &response {
                        Ok(data) => {
                            let ledger_hash = data
                                .stakes
                                .first()
                                .and_then(|x| x.as_ref())
                                .and_then(|x| x.ledger_hash.to_owned());
                            if ledger_hash.is_some() {
                                set_ledger_hash.set(ledger_hash);
                            }
                            response
                        }
                        _ => Err(MyError::ParseError(String::from(
                            "missing epoch information",
                        ))),
                    }
                }
                _ => Err(MyError::ParseError(String::from(
                    "missing epoch information",
                ))),
            }
        },
    );

    let get_data = move || resource.get().and_then(|res| res.ok());

    let table_columns = vec![
        TableColumn {
            column: "Key".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Stake".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Total Stake %".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Delegate".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Delegators".to_string(),
            is_searchable: false,
        },
    ];
    let table_cols_length = table_columns.len();
    let table_columns_clone = table_columns.clone();

    create_effect(move |_| {
        if let Some(data) = get_data() {
            set_metadata.set(Some(TableMetadata {
                total_records: "all".to_string(),
                displayed_records: data.stakes.len() as i64,
            }))
        }
    });

    let get_heading_and_epochs = create_memo(move |_| {
        summary_resource
            .get()
            .and_then(|res| res.ok())
            .map(|sum_data| {
                let curr_epoch = sum_data.epoch as i64;
                let mut section_heading = "Staking Ledger - Epoch ".to_string();
                let mut next_epoch = curr_epoch + 1;
                let mut prev_epoch = curr_epoch - 1;
                let header_epoch = if let Some(qs_epoch) = epoch_sig.get() {
                    if qs_epoch != curr_epoch {
                        next_epoch = qs_epoch + 1;
                        next_epoch = next_epoch.clamp(0, curr_epoch + 1);
                        prev_epoch = qs_epoch - 1;
                        qs_epoch
                    } else {
                        curr_epoch
                    }
                } else {
                    curr_epoch
                };
                section_heading += format!("{}", header_epoch).as_str();
                (
                    section_heading,
                    curr_epoch,
                    next_epoch,
                    prev_epoch,
                    sum_data.slot,
                )
            })
            .unwrap_or(("".to_string(), 0, 0, 0, 0))
    });

    {
        move || {
            let table_columns_clone = table_columns_clone.clone();
            let (section_heading, curr_epoch, next_epoch, prev_epoch, slot) =
                get_heading_and_epochs.get();
            view! {
                <TableSection
                    metadata
                    section_heading=section_heading
                    controls=move || {
                        view! {
                            <EpochButton
                                disabled=prev_epoch < 0
                                text="Previous"
                                style_variant=EpochStyleVariant::Secondary
                                epoch_target=prev_epoch
                            />
                            <EpochButton
                                text="Next"
                                style_variant=EpochStyleVariant::Primary
                                epoch_target=next_epoch
                            />
                        }
                    }

                    additional_info=view! {
                        {match ledger_hash.get() {
                            Some(data) => {
                                view! {
                                    <div class="text-sm text-slate-500">
                                        {convert_to_link(data, "#".to_string())}
                                    </div>
                                }
                                    .into_view()
                            }
                            None => ().into_view(),
                        }}

                        {if next_epoch - 1 == curr_epoch {
                            view! {
                                <div class="text-sm text-dark-blue staking-ledger-percent-complete">
                                    {format!(
                                        "{:.2}% complete ({}/{} slots filled)",
                                        ({ slot } as f64 / { EPOCH_SLOTS } as f64) * 100.0,
                                        { slot },
                                        { EPOCH_SLOTS },
                                    )}

                                </div>
                            }
                                .into_view()
                        } else {
                            ().into_view()
                        }}
                    }
                >

                    <TableContainer>
                        <Table>
                            <TableHeader columns=table_columns_clone.clone()/>
                            <Suspense fallback=move || {
                                view! {
                                    <TableRows data=vec![
                                        vec![LoadingPlaceholder; table_cols_length];
                                        10
                                    ]/>
                                }
                            }>
                                {move || {
                                    get_data()
                                        .map(|data| {
                                            view! { <TableRows data=data.stakes/> }
                                        })
                                }}

                            </Suspense>
                        </Table>
                    </TableContainer>
                </TableSection>
            }
        }
    }
}

#[component]
pub fn EpochButton(
    #[prop(into)] text: String,
    #[prop(optional)] epoch_target: i64,
    #[prop(default = false)] disabled: bool,
    style_variant: EpochStyleVariant,
    #[prop(default=String::new(), into)] href: String,
) -> impl IntoView {
    let button_base_styles = "text-sm rounded-md p-2 h-9 font-semibold ml-2 flex justify-center items-center border border-granola-orange border-[1px]";
    let mut button_variant_styles = match style_variant {
        EpochStyleVariant::Primary => {
            format!("{} {}", button_base_styles, "text-white bg-granola-orange")
        }
        EpochStyleVariant::Secondary => {
            format!("{} {}", button_base_styles, "text-granola-orange bg-white")
        }
    };
    button_variant_styles = match disabled {
        true => format!(
            "{} {}",
            button_variant_styles,
            "bg-slate-100 text-slate-400 border-slate-100 hover:cursor-not-allowed"
        ),
        false => button_variant_styles,
    };

    let query_params_map = use_query_map();
    let navigate = use_navigate();
    let location = use_location();

    if href.clone().is_empty() {
        let handle_click = move |_| {
            if disabled {
                return;
            }
            let pathname = location.pathname.get();
            let mut pm = query_params_map.get();
            pm.insert("epoch".to_string(), epoch_target.to_string());

            logging::log!("{}", pm.to_query_string());
            logging::log!("{}", pathname);

            navigate(
                &format!("{}{}", pathname, pm.to_query_string()),
                Default::default(),
            );
        };
        view! {
            <button on:click=handle_click class=button_variant_styles>
                {text}
            </button>
        }
        .into_view()
    } else {
        view! {
            <a href=href class=button_variant_styles>
                {text}
            </a>
        }
        .into_view()
    }
}
