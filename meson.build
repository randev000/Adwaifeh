project('adwaifeh', 'rust',
          version: '0.1.0',
    meson_version: '>= 0.50.0',
  default_options: [ 'warning_level=2',
                   ],
)

i18n = import('i18n')

dependency('gtk4', version: '>= 4.0.0')
dependency('libadwaita-1', version: '>=1.0.0')

cargo_sources = files(
  'Cargo.toml',
  'Cargo.lock',
)

dependency('glib-2.0', version: '>= 2.66')
dependency('gio-2.0', version: '>= 2.66')

app_id = 'org.randev.Adwaifeh'

subdir('data')
subdir('src')
subdir('po')

meson.add_install_script('build-aux/meson/postinstall.py')
