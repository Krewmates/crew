# 🚢 crew

**CLI de developer experience (DX) que unifica Git + Docker num só lugar.**

![CI](https://github.com/Krewmates/crew/actions/workflows/rust.yml/badge.svg)
![License](https://img.shields.io/badge/license-GPLv3-blue.svg)
![Rust](https://img.shields.io/badge/rust-2024-orange.svg)

`crew` é uma ferramenta de linha de comando escrita em Rust para reduzir o
atrito do dia a dia de desenvolvimento: menus interativos para operações de
Git e, em breve, gerenciamento de containers Docker — tudo sem sair do
terminal e sem precisar decorar flags.

> ⚠️ **Projeto em estágio MVP.** O módulo de Git já é funcional. Os módulos
> de Docker e os atalhos globais (`status`/`up`/`down`) ainda estão em
> desenvolvimento — veja [Status atual](#-status-atual).

---

## ✨ Por que crew?

Trocar de branch, criar branch, checar status de containers, ver logs...
são tarefas pequenas que a gente repete o dia inteiro e que normalmente
exigem lembrar de vários comandos diferentes. O `crew` junta isso em menus
interativos simples, com feedback visual claro (✅ sucesso, ⚠️ aviso, ℹ️ info).

## 📦 Instalação

Ainda não publicado no crates.io — instale a partir do código-fonte:

```bash
git clone https://github.com/Krewmates/crew.git
cd crew
cargo install --path .
```

Isso instala o binário `crew` no seu `~/.cargo/bin` (garanta que está no seu `PATH`).

Alternativamente, para só compilar e testar localmente:

```bash
cargo build --release
./target/release/crew --help
```

### Pré-requisitos

- [Rust](https://rustup.rs/) (edition 2024 / toolchain estável recente)
- `git` instalado e disponível no `PATH`
- Docker instalado e rodando *(necessário apenas para os comandos `crew docker`, ainda em desenvolvimento)*

## 🚀 Uso

```bash
crew --help
```

### `crew git`

Abre um menu interativo com as operações de Git mais comuns:

```bash
crew git
```

Dentro do menu você pode:

- 👀 **Ver a branch atual** (e se há mudanças não commitadas)
- 📋 **Listar todas as branches** locais
- 🌱 **Criar uma nova branch** (com validação de nome)
- 🔀 **Trocar de branch**
- 🚪 Sair

Este é o único módulo 100% funcional hoje.

### `crew docker` 🚧

```bash
crew docker ls                # lista containers
crew docker logs <container>  # mostra logs
crew docker kill <container>  # mata um container
```

> Ainda são *stubs* — a integração real com o Docker (via `bollard`) está
> planejada mas não implementada.

### `crew global` 🚧

```bash
crew global status  # status unificado de Git + Docker
crew global up       # sobe containers selecionados interativamente
crew global down     # derruba containers selecionados interativamente
```

> Também em stub, aguardando implementação.

## ✅ Status atual

| Módulo | Status | Descrição |
|---|---|---|
| `crew git` | ✅ Implementado | Ver branch, listar, criar e trocar de branch, via menu interativo |
| `crew docker` | 🚧 Stub | Estrutura pronta, integração com `bollard` pendente |
| `crew global` | 🚧 Stub | Estrutura pronta, aguardando `status`/`up`/`down` reais |
| Configuração (`crew.toml` ou similar) | ⏳ Planejado | Ainda não iniciado |

## 🏗️ Arquitetura

```
src/
├── main.rs          # entrypoint e dispatch dos comandos
├── cli.rs             # definição da CLI (clap)
├── error.rs            # CrewError / CrewResult
├── config/             # configuração do projeto (a implementar)
├── commands/            # orquestração da regra de negócio
├── git/                  # lógica de domínio Git (via `git` do sistema)
├── docker/               # lógica de domínio Docker (via `bollard`)
└── ui/                    # menus e tabelas interativas (via `inquire`)
```

## 🛠️ Stack técnica

| Crate | Uso |
|---|---|
| `clap` | Parsing da CLI |
| `tokio` | Runtime assíncrono |
| `bollard` | Cliente da API do Docker |
| `inquire` | Prompts e menus interativos no terminal |
| `comfy-table` | Tabelas formatadas |
| `serde` / `serde_yaml` | Parsing de `docker-compose.yml` |
| `thiserror` | Erros tipados |

## 🤝 Contribuindo

1. Faça um fork e crie sua branch a partir de `develop`
2. Antes de abrir um PR, garanta que passa localmente:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo build
   cargo test
   ```
3. Abra o PR contra `develop`

O CI roda automaticamente essas mesmas checagens em todo push/PR para `main` e `develop`.

## 📄 Licença

Distribuído sob a licença **GPLv3**. Veja [`LICENSE`](./LICENSE) para mais detalhes.
