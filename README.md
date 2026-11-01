# Πρόκληση Διαιρετότητας

Εφαρμογή εξάσκησης για τα Μαθηματικά Α΄ Γυμνασίου (κεφάλαιο «Διαιρετότητα»).
Ζωντανά στο **https://maths.savvy-des.com**.

- `public/index.html`: όλη η εφαρμογή σε ένα αρχείο (HTML, CSS, JS).
- `tests/generators.test.mjs`: ελέγχει ότι κάθε γεννήτρια ερωτήσεων βγάζει έγκυρες ερωτήσεις.
- `wrangler.jsonc`: ρυθμίσεις του Cloudflare Worker (στατικά αρχεία και custom domain).

## Πώς γίνεται η δημοσίευση

Κάθε push στο `main` τρέχει το GitHub Actions workflow `.github/workflows/deploy.yml`:
πρώτα τρέχει τα tests, μετά κάνει `wrangler deploy` στο Cloudflare.
Τα pull requests τρέχουν μόνο τα tests.

Χρειάζονται δύο secrets στο repository (Settings → Secrets and variables → Actions):

| Secret | Τιμή |
| --- | --- |
| `CLOUDFLARE_ACCOUNT_ID` | Το Account ID του Cloudflare (dash.cloudflare.com → Workers & Pages, δεξιά στη σελίδα) |
| `CLOUDFLARE_API_TOKEN` | API token με το template «Edit Cloudflare Workers» |

## Τοπικά

```bash
npm install
npm test
npx wrangler dev
```
