#!/usr/bin/env python3
from distutils.util import get_platform
from zipfile import ZipFile, ZipInfo
from os import makedirs

VERSION = '0.1.0'
PLAT_TAG = get_platform().replace('-', '_').replace('.', '_')

METADATA = f"""Metadata-Version: 2.1
Name: espadrille
Version: {VERSION}
"""
WHEEL = """Wheel-Version: 1.9
Generator: espadrille
Root-Is-Purelib: false
Tag: py3-none-{PLAT_TAG}
"""
RECORD = f"""espadrille-{VERSION}.data/scripts/espadrille
espadrille-{VERSION}.dist-info/METADATA
espadrille-{VERSION}.dist-info/WHEEL
"""

def write_folder(z, path):
    info = ZipInfo(path)
    info.external_attr = 16
    z.writestr(info, b'')

makedirs('dist', exist_ok=True)

with ZipFile(f'dist/espadrille-{VERSION}-py3-none-{PLAT_TAG}.whl', 'w') as z:
    z.write('target/release/espadrille', f'espadrille-{VERSION}.data/scripts/espadrille')
    z.writestr(f'espadrille-{VERSION}.dist-info/METADATA', METADATA)
    z.writestr(f'espadrille-{VERSION}.dist-info/WHEEL', WHEEL)
    z.writestr(f'espadrille-{VERSION}.dist-info/RECORD', RECORD)