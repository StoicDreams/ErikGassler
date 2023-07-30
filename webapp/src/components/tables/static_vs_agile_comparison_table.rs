use crate::prelude::*;

#[function_component(StaticVsAgileComparisonTable)]
pub(crate) fn static_vs_agile_comparison_table() -> Html {
    let dev_salary = use_state(|| Decimal::from_u32(1880000).unwrap());
    let custom_developers = use_state(|| 4u8);
    let custom_months_to_launch = use_state(|| Decimal::from_u8(6).unwrap());

    let dev_salary_str = use_state(|| "188000".to_string());
    let custom_developers_str = use_state(|| "4".to_string());
    let custom_months_to_launch_str = use_state(|| "6".to_string());

    let onchange_ds = {
        let value = dev_salary.clone();
        let input = dev_salary_str.clone();
        Callback::from(move |_| {
            value.set(Decimal::from_str(&input).unwrap());
        })
    };
    let onchange_cd = {
        let value = custom_developers.clone();
        let input = custom_developers_str.clone();
        Callback::from(move |_| {
            if let Ok(input) = u8::from_str(&input) {
                value.set(input);
            };
        })
    };
    let onchange_cmtl = {
        let value = custom_months_to_launch.clone();
        let input = custom_months_to_launch_str.clone();
        Callback::from(move |_| {
            if let Ok(input) = Decimal::from_str(&input) {
                value.set(input);
            };
        })
    };
    let columns = vec![
        TableColumns::<WorkflowDetails>::new(
            "Project".to_string(),
            |data| html! {data.name.to_string()},
        ),
        TableColumns::<WorkflowDetails>::new(
            "Company".to_string(),
            |data| html! {data.company.to_string()},
        ),
        TableColumns::<WorkflowDetails>::new(
            "Workflow".to_string(),
            |data| html! {data.workflow.to_string()},
        ),
        TableColumns::<WorkflowDetails>::new(
            "Developers".to_string(),
            |data| html! {data.developers.to_string()},
        )
        .align(LeftCenterRight::Center),
        TableColumns::<WorkflowDetails>::new("Time to Launch".to_string(), |data| {
            let ttl = data.time_to_launch;
            if ttl < Decimal::from_u8(1).unwrap() {
                return html! {format!("{} Weeks", (ttl * Decimal::from_u8(100).unwrap()).round())};
            }
            html! {format!("{} Months", ttl)}
        })
        .align(LeftCenterRight::Center),
        TableColumns::<WorkflowDetails>::new(
            "Update Frequency".to_string(),
            |data| html! {data.update_frequency.to_string()},
        )
        .align(LeftCenterRight::Center),
        TableColumns::<WorkflowDetails>::new("Cost to Delivery".to_string(), move |data| {
            html! {format!("${}", format_decimal(data.cost_to_deliver.round(),0))}
        })
        .align(LeftCenterRight::Right),
        TableColumns::<WorkflowDetails>::new("Monthly Cost".to_string(), |data| {
            html! {format!("${}", format_decimal(data.monthly_cost.round(),0))}
        })
        .align(LeftCenterRight::Right),
        TableColumns::<WorkflowDetails>::new("Yearly Cost".to_string(), |data| {
            html! {format!("${}", format_decimal(data.yearly_cost.round(), 0))}
        })
        .align(LeftCenterRight::Right),
    ];
    let data = vec![
        WorkflowDetails::new(
            "Your Project".to_string(),
            "Your Company".to_string(),
            "Your Workflow".to_string(),
            *custom_developers,
            *custom_months_to_launch,
            "Daily? Weekly? Monthly?".to_string(),
            *dev_salary,
        ),
        WorkflowDetails::new(
            "Task Story".to_string(),
            "Stoic Dreams".to_string(),
            "Agile, Continuous Delivery".to_string(),
            1,
            Decimal::from_str("0.02").unwrap(),
            "Multiple-Times-Daily".to_string(),
            *dev_salary,
        ),
        WorkflowDetails::new(
            "IAAS Tool".to_string(),
            "Costco".to_string(),
            "Scrum / DevOps / Sprints".to_string(),
            5,
            Decimal::from_str("36").unwrap(),
            "Weekly-Monthly".to_string(),
            *dev_salary,
        ),
        WorkflowDetails::new(
            "Reporting".to_string(),
            "Microsoft".to_string(),
            "Scrum / DevOps / Sprints".to_string(),
            8,
            Decimal::from_str("24").unwrap(),
            "Weekly-Monthly".to_string(),
            *dev_salary,
        ),
        WorkflowDetails::new(
            "Reporting".to_string(),
            "GSC".to_string(),
            "Agile, Continuous Delivery".to_string(),
            1,
            Decimal::from_str("0.02").unwrap(),
            "Multiple-Times-Daily".to_string(),
            *dev_salary,
        ),
        WorkflowDetails::new(
            "Task Story".to_string(),
            "Stoic Dreams".to_string(),
            "Agile, Continuous Delivery".to_string(),
            1,
            Decimal::from_str("0.01").unwrap(),
            "Multiple-Times-Daily".to_string(),
            *dev_salary,
        ),
        WorkflowDetails::new(
            "Callcenter Tool".to_string(),
            "SoftRock".to_string(),
            "Agile, Continuous Integration".to_string(),
            1,
            Decimal::from_str("2").unwrap(),
            "Daily-Weekly".to_string(),
            *dev_salary,
        ),
    ];
    html!(
        <Paper class="d-flex justify-center align-center flex-column" elevation={ELEVATION_STANDARD}>
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    "This table showcases differences in development costs by comparing development times based on real-world projects I've worked on in some capacity in the various companies I've worked for. Developer counts are an estimated average due to fluctuating team sizes during development.",
                    "Configure these fields to match your current project details to compare with the projects below."
                )}
            </Paper>
            <Paper>
                <Paper class="d-flex mt-3 mb-3 flex-wrap flex-gap1 justify-center align-center">
                    <span>{"Your Project Details:"}</span>
                    <InputText value={dev_salary_str} name="Avg Developer Salary" onchange={onchange_ds} />
                    <InputText value={custom_developers_str} name="Developer Count" onchange={onchange_cd} />
                    <InputText value={custom_months_to_launch_str} name="Months to Launch" onchange={onchange_cmtl} />
                </Paper>
                {Table::<WorkflowDetails>::new(columns).add_class("mt-3 mb-3".to_string()).bordered().elevation(ELEVATION_STANDARD).render(data)}
            </Paper>
        </Paper>
    )
}

#[derive(PartialEq)]
struct WorkflowDetails {
    name: String,
    company: String,
    workflow: String,
    developers: u8,
    time_to_launch: Decimal,
    update_frequency: String,
    dev_salary: Decimal,
    yearly_cost: Decimal,
    monthly_cost: Decimal,
    cost_to_deliver: Decimal,
}
impl WorkflowDetails {
    pub fn new(
        name: String,
        company: String,
        workflow: String,
        developers: u8,
        time_to_launch: Decimal,
        update_frequency: String,
        dev_salary: Decimal,
    ) -> Self {
        let yearly_cost = (dev_salary * Decimal::from_u8(developers).unwrap()).round();
        let monthly_cost = yearly_cost / Decimal::from_u8(12).unwrap();
        let weekly_cost = yearly_cost / Decimal::from_u8(52).unwrap();
        let cost_to_deliver = if time_to_launch < Decimal::from_u8(1).unwrap() {
            ((time_to_launch * Decimal::from_u8(100).unwrap()) * weekly_cost).round_dp(2)
        } else {
            (monthly_cost * time_to_launch).round_dp(2)
        };
        Self {
            name,
            company,
            workflow,
            developers,
            time_to_launch,
            update_frequency,
            dev_salary,
            yearly_cost,
            monthly_cost,
            cost_to_deliver,
        }
    }
}
