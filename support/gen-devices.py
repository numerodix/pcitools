#!/usr/bin/env python

import re
import sys


rx_vendor = re.compile('^([0-9a-fA-F]{4})  (.*)$')
rx_device = re.compile('^\t([0-9a-fA-F]{4})  (.*)$')

def main(id_file, tmpl_file):
    triples = []

    with open(id_file, 'r') as infile:
        vendor_id = None
        for line in infile:
            match = rx_vendor.match(line)
            if match:
                vendor_id, _ = match.groups()
                continue

            match = rx_device.match(line)
            if match:
                device_id, name = match.groups()
                triples.append((vendor_id, device_id, name))

    with open(tmpl_file, 'r') as outfile:
        content = outfile.read()

    indent = '        '
    lines = []
    for vendor_id, device_id, name in triples:
        name = name.replace('"', '\\"')
        line = f'{indent}((0x{vendor_id}, 0x{device_id}), "{name}"),'
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