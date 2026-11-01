# Πρόκληση Διαιρετότητας

Εφαρμογή εξάσκησης για τα Μαθηματικά Α΄ Γυμνασίου, σε όλες τις ενότητες του κεφαλαίου
«Διαιρετότητα». Ζωντανά στο **https://maths.savvy-des.com**.

Είναι γραμμένη σε **Rust** και τρέχει ως Cloudflare Worker (WebAssembly).
Οι ερωτήσεις φτιάχνονται, ελέγχονται και διορθώνονται στον server, οπότε οι
απαντήσεις δεν φτάνουν ποτέ στον browser του μαθητή πριν απαντήσει.

## Ενότητες

| Ενότητα του βιβλίου | Κατηγορίες στην εφαρμογή |
| --- | --- |
| Ευκλείδεια Διαίρεση | Ευκλείδεια διαίρεση · Διαιρέτες και πολλαπλάσια |
| Ιδιότητες Διαιρετότητας | Ιδιότητες διαιρετότητας |
| Κριτήρια Διαιρετότητας | Κριτήρια διαιρετότητας |
| Πρώτοι και Σύνθετοι Αριθμοί | Πρώτοι και σύνθετοι · Πρώτοι παράγοντες |
| ΜΚΔ και ΕΚΠ | ΜΚΔ & ΕΚΠ με δυνάμεις · ΜΚΔ & ΕΚΠ αριθμών · Προβλήματα ΜΚΔ · Προβλήματα ΕΚΠ (ώρες, χρονιές) |
| Δραστηριότητες Ενότητας / Εμπλουτισμού | Προκλήσεις · Κριτήρια 7, 11, 19 |

## Δομή

```
crates/quiz-core   γεννήτριες ερωτήσεων, έλεγχος απαντήσεων, ανάλυση λαθών (καθαρή Rust, με tests)
crates/site        ο Worker: /api/* και το τμήμα «Για τον εκπαιδευτικό»
public/            η σελίδα (HTML/CSS/JS), σερβίρεται από Workers Assets
wrangler.jsonc     ρυθμίσεις Cloudflare (custom domain, όριο προσπαθειών PIN)
```

### API

| Route | |
| --- | --- |
| `GET /api/categories` | κατηγορίες και ονομασίες λαθών |
| `GET /api/question?cat=&seed=` | ερώτηση χωρίς απαντήσεις |
| `GET /api/hint?cat=&seed=&n=` | βοήθεια `n` (0–3) |
| `POST /api/check` | έλεγχος, ανάλυση λάθους, λύση |
| `POST /api/teacher/login` | PIN → token 12 ωρών (μέχρι 6 προσπάθειες/λεπτό) |
| `GET /api/teacher/resources` | διαγώνισμα και λύσεις (με token) |
| `GET /api/teacher/worksheet?cat=&n=` | φύλλο εργασίας με απαντήσεις (με token) |

## Για τον εκπαιδευτικό

Το τμήμα «Για τον εκπαιδευτικό» ζητά PIN. Το PIN είναι Worker secret με όνομα
`TEACHER_PIN` και δεν υπάρχει πουθενά στον κώδικα. Για να το αλλάξετε:

```bash
npx wrangler secret put TEACHER_PIN
```

ή από το Cloudflare dashboard: Workers & Pages → maths-savvy-des → Settings → Variables and Secrets.

## Δημοσίευση

Κάθε push στο `main` τρέχει το `.github/workflows/deploy.yml`:
`cargo fmt`, `clippy`, `cargo test`, μετά build σε WebAssembly και `wrangler deploy`,
και στο τέλος ένα γρήγορο έλεγχο ότι η σελίδα απαντά. Τα pull requests τρέχουν μόνο τους ελέγχους.

Secrets του repository: `CLOUDFLARE_API_TOKEN` (template «Edit Cloudflare Workers») και `CLOUDFLARE_ACCOUNT_ID`.

## Τοπικά

```bash
rustup target add wasm32-unknown-unknown
npm install
cargo test -p quiz-core
echo "TEACHER_PIN=123456" > .dev.vars
npx wrangler dev
```
