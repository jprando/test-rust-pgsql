# RUST + POSTGRES

Projeto pra praticar e aprender mais sobre acessar dados em um banco de dados postgres via rust lang.

## Configurando a conexão

Pode ser utilizado variáveis de ambiente para informa em qual instancia do postgres você quer se conectar.

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

Você também pode definir essas variáveis em um arquivo de configuração chamado `.env`.  
O arquivo deve estar localizado na mesma pasta do binário a ser executado exemplo:

```
~/minha-pasta
    |-- test-rust-pgsql
    |-- .env
```

com o seguinte conteúdo:

```env
PGHOST=meuservidor
PGPORT=5432
PGUSER=meuusuario
PGPASS=minhasenhasupersecreta
PGDATA=meubancodedados
```

nos arquivos deste repositório há um exemplo:  
[💾 .env.example](/.env.example)

Uma vez o arquivo `.env` criado, não é necessário utilizar variáveis de ambiente ou passar como variáveis de contexto para executar o programa, mas se o fizer e existir a configuração tanto no arquivo `.env` quanto no ambiente, prevalece o valor da variável de ambiente.

## fn main () { show_me_the_code(); }

[💾 main.rs](/src/main.rs)
