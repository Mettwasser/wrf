generate:
    spacetime generate --lang typescript --out-dir frontend/src/lib/module_bindings --module-path spacetimedb

publish *args:
    spacetime publish -y {{ args }}
