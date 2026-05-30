p = r'C:\PRIVATE\github\bitwill\Free_prod.htm'
lines = open(p, encoding='utf-8').read().split('\n')
out = []
for i, ln in enumerate(lines, 1):
    if 'â' in ln or 'Ã' in ln or 'ð' in ln or 'Å' in ln or '\x9f' in ln or 'Â' in ln:
        # show line number, a trimmed view, and repr of the non-ascii bits
        na = ''.join(c for c in ln if ord(c) > 127)
        out.append(f'{i}: NONASCII={na!r}')
        out.append(f'    TEXT={ln.strip()[:120]!r}')
open(r'C:\PRIVATE\github\bitwill\scan_out.txt', 'w', encoding='utf-8').write('\n'.join(out) if out else 'NONE')
