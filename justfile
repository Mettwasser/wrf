##################################
#              Dev               #
##################################

generate:
    spacetime generate --lang typescript --out-dir frontend/src/lib/module_bindings --module-path spacetimedb

dev *args:
    spacetime start {{ args }}

sql *args:
    spacetime sql --interactive -s "127.0.0.1:3000" {{ args }}

publish *args:
    spacetime publish -y -s "127.0.0.1:3000" {{ args }}

logs *args:
    spacetime logs -s "127.0.0.1:3000" --follow {{ args }}

##################################
#              Prod              #
##################################

sql-prod *args:
    spacetime sql --interactive {{ args }}

publish-prod *args:
    spacetime publish -y {{ args }}
