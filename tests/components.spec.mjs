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
        const link = page.locator('app-books').getByRole('link', { name: 'my announcement' });
        await expect(link).toHaveCount(1);
        await link.click();
        await page.waitForURL('**/blogs/book_published_for_case_continuous_agile_software_engineering');
        await expect(page.locator('webui-page-not-found')).toHaveCount(0);
    });

    test('without link-to-blog there is no link to the announcement', async ({ page }) => {
        await openPage(page, announcement);
        await expect(page.locator('app-books')).toContainText('Kindle');
        await expect(page.locator('app-books').getByRole('link', { name: 'my announcement' })).toHaveCount(0);
    });
});
