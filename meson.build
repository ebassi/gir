# SPDX-FileCopyrightText: 2022  Emmanuele Bassi
# SPDX-License-Identifier: MIT

project('gir', 'rust',
  version: '0.0.1',
  license: ['MIT'],
  meson_version: '>= 0.62.0',
)

cargo = find_program('cargo', required: true)

cargo_sources = files(
  'Cargo.toml',
  'Cargo.lock',
)

cargo_options = [
  [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ],
  [ '--target-dir', meson.project_build_root() ],
]

if get_option('profile') == 'default'
  cargo_options += [ '--release' ]
  rust_target = 'release'
  message('Building in release mode')
else
  rust_target = 'debug'
  message('Building in debug mode')
endif

cargo_env = [ 'CARGO_HOME=' + meson.project_build_root() / 'cargo-home' ]

cargo_gir = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    'env',
    cargo_env,
    cargo, 'build',
    cargo_options,
    '&&',
    'cp', rust_target / meson.project_name(), '@OUTPUT@',
  ],
)
