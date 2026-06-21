# Bitcoin Legacy Kit — MVP backend

Zero-dependency Node server that serves the landing pages and stores gift-form
submissions securely.

## Run

```bash
node server/server.js
```

Then open **http://localhost:3000/Free_prod.htm** (not `file://`, so the form
posts same-origin).

Custom port:

```bash
# PowerShell
$env:PORT=8080; node server/server.js
```

## What it does

- Serves `Free_prod.htm`, `letterbox.htm`, CSS/JS/images from the project root.
- `POST /api/gift` validates the submission and appends it to
  `server/data/submissions.jsonl` (one JSON object per line).

## Stored fields

| field        | notes                                             |
|--------------|---------------------------------------------------|
| id           | random UUID                                       |
| createdAt    | ISO timestamp                                     |
| gift         | `bags` \| `sim` \| `vault`                        |
| name         | optional                                          |
| parcelLocker | required (min 6 chars)                            |
| email        | **only** for `vault` (the lucky draw); else `null`|

## Security (MVP)

- Same-origin only — no CORS is opened.
- Security headers on every response (nosniff, DENY frames, CSP, no-referrer).
- 8 KB request-body cap; per-IP rate limit (12 submissions/hour).
- Strict allow-list validation + control-char stripping on all fields.
- Email accepted/stored **only** for the vault draw.
- Path-traversal-safe, extension-allow-listed static serving.
- No personal data printed to logs; `data/` is git-ignored.

## Production notes (beyond MVP)

- Terminate TLS in front (reverse proxy / platform) so traffic is HTTPS.
- Move storage to a managed DB and encrypt the email column at rest.
- Add a retention job (the page promises the data isn't kept after shipping).
