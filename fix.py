import ftfy

p = r'C:\PRIVATE\github\bitwill\Free_prod.htm'
t = open(p, encoding='utf-8').read()

fixed = ftfy.fix_encoding(t)
# straighten any smart quotes (invalid as JS string delimiters)
fixed = (fixed.replace('‘', "'").replace('’', "'")
              .replace('“', '"').replace('”', '"'))

# verify it's valid UTF-8 before writing
fixed.encode('utf-8')
open(p, 'w', encoding='utf-8').write(fixed)

rep = ['valid_utf8=YES', 'lines=' + str(fixed.count(chr(10)))]
for k, v in {
    'arrow': '→', 'emdash': '—', 'box': '\U0001F4E6', 'lock': '\U0001F512',
    'signal': '\U0001F4F6', 'plane': '✈', 'gift': '\U0001F381',
    'cross': '✕', 'check': '✓', 'middot': '·',
    'replacement': '�', 'mojibake_a': 'â', 'mojibake_A': 'Ã',
    'curly': '’', 'ghost_btn': 'class="btn-ghost"', 'data_download': 'data-download',
    'dl_binding': "querySelectorAll('[data-download]')", 'iife_close': '})();',
}.items():
    rep.append(k + '=' + str(fixed.count(v)))
open(r'C:\PRIVATE\github\bitwill\fix_report.txt', 'w', encoding='utf-8').write('\n'.join(rep))
