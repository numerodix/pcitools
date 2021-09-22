#!/usr/bin/env python

import re
import sys


rx_vendor = re.compile('^([0-9a-fA-F]{4})  (.*)$')

def main(id_file, tmpl_file):
    pairs = []

    with open(id_file, 'r') as infile:
        for line in infile:
            match = rx_vendor.match(line)
            if match:
                id, name = match.groups()
                pairs.append((id, name))

    with open(tmpl_file, 'r') as outfile:
        content = outfile.read()

    indent = '        '
    lines = []
    for id, name in pairs:
        name = name.replace('"', '\\"')
        line = f'{indent}(0x{id}, "{name}"),'
        lines.append(line)

    lines = [f'{indent}// START'] + lines + [f'{indent}// END']
    block = '\n'.join(lines)
    content = re.sub(f'(?ms){indent}// START.*{indent}// END', block, content)

    with open(tmpl_file, 'w') as outfile:
        outfile.write(content)


if __name__ == '__main__':
    id_file = sys.argv[1]
    tmpl_file = sys.argv[2]
    main(id_file, tmpl_file)