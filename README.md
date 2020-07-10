# RUST + POSTGRES

Projeto pra praticar e aprender mais sobre acessar dados em um banco de dados postgres via rust lang.

## Configurando a conexão

Pode ser utilizado variaveis de ambiente para informa em qual instancia do postgres voce quer se conectar.

- `PGHOST` nome ou ip do servidor ( padrão: 127.0.0.1 )
- `PHPORT` porta do serviço ( padrão: 5432 )
- `PGUSER` nome do usuário ( padrão: postgres )
- `PGPASS` senha
- `PGDATA` nome do banco de dados ( padrão: postgres )

Exemplo de utilização:

```bash
env PGHOST=meuservidor \
	PGPORT=5432 \
	PGUSER=meuusuario \
	PGPASS=minhasenhasupersecreta \
	PGDATA=meubandodedados \
./test-rust-pgsql
```

## fn main () { show_me_the_code(); }

[main.rs](/src/main.rs)