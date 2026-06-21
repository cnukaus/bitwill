
  What I built

  Front-end (Free_prod.htm) — dynamic gift form:
  - Bags / SIM: submit name + parcel-locker address only. No email field shown or sent.
  - Vault: selecting it reveals an Email field that becomes mandatory (red "— required for the lucky draw" label).
  It validates with an email regex and blocks submit if missing/invalid. The email is only collected and only
  sent when vault is chosen — switching back to bags/sim hides it again (emailWrap.hidden = (m !== 'vault')).
  - The form now does a real fetch POST to /api/gift, disables the button while sending, shows a success view on
  {ok:true}, and shows an inline red error if the save fails.

  Back-end (new server/ folder): a zero-dependency Node server (server.js, built-in http only — no npm install).

  Run it:
  node server/server.js
  then open http://localhost:3000/Free_prod.htm (served same-origin so the form posts cleanly). Submissions append
  to server/data/submissions.jsonl.

  Security measures (all tested):

  ┌────────────────────────────────────────┬──────────────────────────────────┐
  │                  Test                  │              Result              │
  ├────────────────────────────────────────┼──────────────────────────────────┤
  │ vault without email                    │ 400 invalid_email ✅             │
  ├────────────────────────────────────────┼──────────────────────────────────┤
  │ vault with email                       │ saved ✅                         │
  ├────────────────────────────────────────┼──────────────────────────────────┤
  │ bags/sim store email:null              │ ✅                               │
  ├────────────────────────────────────────┼──────────────────────────────────┤
  │ invalid gift / short parcel / bad JSON │ 400 ✅                           │
  ├────────────────────────────────────────┼──────────────────────────────────┤
  │ path traversal ../../Windows/win.ini   │ 404 ✅                           │
  ├────────────────────────────────────────┼──────────────────────────────────┤
  │ non-whitelisted symbiosis.txt          │ 404 ✅ (other files not exposed) │
  └────────────────────────────────────────┴──────────────────────────────────┘

  Plus: 8 KB body cap, per-IP rate limit (12/hr), security headers + CSP on every response, control-char stripping
  & length caps on all fields, data/ git-ignored, file written 0600, and no PII in logs. server/README.md
  documents running it and the production hardening steps (TLS termination, encrypted DB, retention job).

  2. The .jsonl exposure question — answered + hardened

  Do we rely on filesystem permissions? No. OS file permissions only stop other logged-in users on the machine — they do nothing about HTTP. Whether an internet user can fetch the file is decided entirely by the server's routing logic.

  Were users able to see it? The database itself, no (.jsonl was never in the serve allow-list → 404). But I found a real leak: the server's own source (/server/server.js) was fetchable because .js is allow-listed. Now closed.

  The fix (two layers of defense-in-depth):
  1. Extension allow-list — only .htm/.css/.js/.svg/... are ever served; .jsonl, .md, .json return 404.
  2. New: the entire server/ directory (source code and data/) is explicitly blocked from web serving, regardless of extension.