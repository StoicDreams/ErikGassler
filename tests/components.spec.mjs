import { test, expect, openPage } from './support/site.mjs';

test.describe('app-books', () => {
    const editions = { Kindle: 'https://www.amazon.com/dp/B0C4FX1F1S', Paperback: 'https://www.amazon.com/dp/B0C47LV1Y8', Hardcover: 'https://www.amazon.com/dp/B0C47YGGYH' };

    const announcement = '/blogs/book_published_for_case_continuous_agile_software_engineering';

    test('links to each edition of the book', async ({ page }) => {
        await openPage(page, announcement);
        const books = page.locator('app-books');
        for (const [label, href] of Object.entries(editions)) {
            await expect(books.getByRole('link', { name: label, exact: true })).toHaveAttribute('href', href);
        }
        await expect(books.locator('img')).toHaveAttribute('alt', /\w/);
    });

    test('with link-to-blog it links to the announcement, which exists', async ({ page }) => {
        // No page uses link-to-blog at the moment, so add one to an existing page.
        await openPage(page, '/publications');
        await page.evaluate(() => {
            const books = document.createElement('app-books');
            books.setAttribute('link-to-blog', '');
            document.querySelector('webui-page-segment').after(books);
        });
        const link = page.locator('app-books').getByRole('link', { name: 'my accouncement blog' });
        await expect(link).toHaveCount(1);
        await link.click();
        await page.waitForURL('**/blogs/book_published_for_case_continuous_agile_software_engineering');
        await expect(page.locator('webui-page-not-found')).toHaveCount(0);
    });

    test('without link-to-blog there is no link to the announcement', async ({ page }) => {
        await openPage(page, announcement);
        await expect(page.locator('app-books')).toContainText('Kindle');
        await expect(page.locator('app-books').getByRole('link', { name: 'my accouncement blog' })).toHaveCount(0);
    });
});

test.describe('app-static-vs-agile-comp-table', () => {
    const table = page => page.locator('app-static-vs-agile-comp-table webui-table');
    const rows = page => page.evaluate(() => webui.getData('page-report'));
    const field = (page, label) => page.locator(`app-static-vs-agile-comp-table webui-input-text[label="${label}"] input`);
    const dollars = amount => `$${amount.toLocaleString('en-US')}`;
    const parseMonths = text => text.endsWith('Weeks') ? parseFloat(text) / 4 : parseFloat(text);

    test.beforeEach(async ({ page }) => {
        await openPage(page, '/services/consulting');
        await expect.poll(async () => (await rows(page))?.length).toBe(5);
    });

    test('shows your project first, using the default salary, team, and time', async ({ page }) => {
        const [yours] = await rows(page);
        expect(yours).toMatchObject({ Project: 'Your Project', Developers: '3', TimeToLaunch: '18 Months' });
        expect(yours.CostToDelivery).toBe(dollars(150000 * 3 * 18));
        expect(yours.MonthlyCost).toBe(dollars(150000 * 3));
        expect(yours.YearlyCost).toBe(dollars(150000 * 3 * 12));
        await expect(table(page)).toContainText('Your Project');
    });

    test('changing the fields recalculates your project', async ({ page }) => {
        await field(page, 'Avg Developer Salary').fill('$100,000');
        await field(page, 'Developer Count').fill('2');
        await field(page, 'Months to Launch').fill('6');
        await expect.poll(async () => (await rows(page))[0]).toMatchObject({
            Developers: '2', TimeToLaunch: '6 Months', CostToDelivery: dollars(100000 * 2 * 6), MonthlyCost: dollars(100000 * 2),
        });
    });

    test('a field that is not a number leaves your project as it was', async ({ page }) => {
        const before = (await rows(page))[0];
        await field(page, 'Developer Count').fill('none');
        await page.waitForTimeout(200);
        expect((await rows(page))[0]).toEqual(before);
    });

    test('every example costs its developers times its time to launch', async ({ page }) => {
        test.fail(true, 'Known bug: the Big-Tech Employer row shows 24 months to launch, but its cost uses 36 months');
        const salary = await page.evaluate(() => webui.getData('page-dev-salary'));
        const wrong = (await rows(page)).filter(row =>
            row.CostToDelivery !== dollars(Math.trunc(salary * Number(row.Developers) * parseMonths(row.TimeToLaunch))));
        expect(wrong.map(row => `${row.Project} at ${row.Company}`)).toEqual([]);
    });
});
