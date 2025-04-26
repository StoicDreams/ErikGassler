/* Comparison table between static versus agile development costs */
"use strict"
{
    const formatDollar = new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' });
    let devSalary = 150000;
    let myMonths = 18;
    let myDevCount = 3;
    function calcCosts(devCount, months) {
        let total = devSalary * devCount * months;
        return formatDollar.format(total).split('.')[0];
    }
    webui.define("app-static-vs-agile-comp-table", {
        setPageDevSalary: function (value) {
            let salary = parseFloat(`${(value || '0')}`.replace(/[^0-9\.]+/g, ''));
            if (!salary) return;
            if (salary === devSalary) return;
            devSalary = salary;
            webui.setData('page-dev-salary', salary);
            this.buildReport();
        },
        setPageDevCount: function (value) {
            let count = parseInt(`${(value || '0')}`.replace(/[^0-9\.]+/g, ''));
            if (!count) return;
            if (count === myDevCount) return;
            myDevCount = count;
            webui.setData('page-dev-count', count);
            this.buildReport();
        },
        setPageMonthsToLaunch: function (value) {
            let months = parseFloat(`${(value || '0')}`.replace(/[^0-9\.]+/g, ''));
            if (!months) return;
            if (months === myMonths) return;
            myMonths = months;
            webui.setData('page-months-to-launch', months);
            this.buildReport();
        },
        keyKeyValue: function (value, key) {
            switch (key) {
                case 'page-dev-salary':
                    this.setDevSalary(value);
                    break;
                case 'page-dev-count':
                    this.setDevCount(value);
                    break;
                case 'page-months-to-launch':
                    this.setMonthsToLaunch(value);
                    break;
                default:
                    console.log('unknown key', key);
                    break;
            }
        },
        buildReport: function () {
            webui.setData('page-report', [
                {
                    Project: 'Your Project',
                    Company: 'Your Company',
                    Workflow: 'Your Workflow',
                    Developers: `${myDevCount}`,
                    TimeToLaunch: `${myMonths} Months`,
                    UpdateFrequency: 'Daily? Weekly? Monthly?',
                    CostToDelivery: calcCosts(myDevCount, myMonths),
                    MonthlyCost: calcCosts(myDevCount, 1),
                    YearlyCost: calcCosts(myDevCount, 12)
                },
                {
                    Project: 'IAAS Tool',
                    Company: 'Mid-Size Employer',
                    Workflow: 'Scrum / DevOps / Sprints',
                    Developers: '5',
                    TimeToLaunch: '36 Months',
                    UpdateFrequency: 'Weekly-Monthly',
                    CostToDelivery: calcCosts(5, 36),
                    MonthlyCost: calcCosts(5, 1),
                    YearlyCost: calcCosts(5, 12)
                },
                {
                    Project: 'Analytics Reporting',
                    Company: 'Big-Tech Employer',
                    Workflow: 'Scrum / DevOps / Sprints',
                    Developers: '8',
                    TimeToLaunch: '24 Months',
                    UpdateFrequency: 'Weekly-Monthly',
                    CostToDelivery: calcCosts(8, 36),
                    MonthlyCost: calcCosts(8, 1),
                    YearlyCost: calcCosts(8, 12)
                },
                {
                    Project: 'Analytics Reporting',
                    Company: 'Startup Employer',
                    Workflow: 'CASE',
                    Developers: '1',
                    TimeToLaunch: '2 Weeks',
                    UpdateFrequency: 'Multiple-Times-Daily',
                    CostToDelivery: calcCosts(1, 0.5),
                    MonthlyCost: calcCosts(1, 1),
                    YearlyCost: calcCosts(1, 12)
                },
                {
                    Project: 'Callcenter Tool',
                    Company: 'Startup Employer',
                    Workflow: 'CASE',
                    Developers: '1',
                    TimeToLaunch: '2 Months',
                    UpdateFrequency: 'Daily-Weekly',
                    CostToDelivery: calcCosts(1, 2),
                    MonthlyCost: calcCosts(1, 1),
                    YearlyCost: calcCosts(1, 12)
                }
            ]);
        },
        connected: (t) => {
            t.addDataset('subscribe', 'page-dev-salary|page-dev-count|page-months-to-launch');
            t.setAttribute('data-setter', 'key-value');
            setTimeout(() => {
                t.buildReport();
            }, 400);
            webui.setData('page-dev-salary', devSalary);
            webui.setData('page-dev-count', myDevCount);
            webui.setData('page-months-to-launch', myMonths);
            t.innerHTML = webui.parseWebuiMarkdown(`
    <webui-flex justify="center" align="center" column elevation="10">
        <webui-page-segment elevation="10">
            This table showcases differences in development costs by comparing development times based on real-world projects I've worked on in some capacity in the various companies I've worked for. Developer counts are an estimated average due to fluctuating team sizes during development.
            Configure these fields to match your current project details to compare with the projects below.
        </webui-page-segment>
        <webui-paper>
            <h4>Your Project Details:</h4>
            <webui-quote theme="tertiary" cite="Erik Gassler">
                The following Employer examples are approximations from real-world projects I worked on at former employers.
            </webui-quote>
            <webui-flex wrap gap="1" justify="center" align="center" class="mt-3 mb-3">
                <webui-input-text compact data-subscribe="page-dev-salary:value" data-trigger="page-dev-salary" label="Avg Developer Salary"></webui-input-text>
                <webui-input-text compact data-subscribe="page-dev-count:value" data-trigger="page-dev-count" label="Developer Count"></webui-input-text>
                <webui-input-text compact data-subscribe="page-months-to-launch:value" data-trigger="page-months-to-launch" label="Months to Launch"></webui-input-text>
            </webui-flex>
            <webui-table theme="success" columns="Project;Company;Workflow;Developers;Time to Launch;Update Frequency;Cost to Delivery;Monthly Cost;Yearly Cost" class="mt-3 mb-3" bordered data-subscribe="page-report:setData"></webui-table>
        </webui-paper>
    </webui-flex>
    `, true);
        }
    });
}