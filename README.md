# NexOS

**NexOS** é um sistema operacional moderno e inovador, criado com o objetivo de repensar a experiência do usuário e a arquitetura tradicional de sistemas baseados em Unix. Desenvolvido do zero em [Rust](https://www.rust-lang.org/), NexOS combina a robustez e segurança da linguagem com uma abordagem minimalista e eficiente para criar um ambiente otimizado para Desktop que é ao mesmo tempo poderoso e fácil de usar.

O objetivo do NexOS é oferecer uma experiência de sistema operacional que se sente moderna e ágil, eliminando a complexidade desnecessária das distribuições tradicionais de Linux. Com um foco na simplicidade, desempenho e usabilidade, NexOS pretende ser o "Linux se fosse feito em 2024."

## Principais Características

- **Compositor Wayland Integrado:** NexOS incorpora diretamente um compositor Wayland dentro do kernel, proporcionando um ambiente gráfico rápido e responsivo sem a necessidade de camadas adicionais e comunicação entre processos (IPC).
- **100% Compatível com Binários de Linux:** NexOS é totalmente compatível com binários de Linux, permitindo que qualquer programa que utilize Wayland para a GUI funcione perfeitamente. Além disso, o suporte ao XWayland garante que aplicações antigas ainda baseadas em Xorg também possam ser executadas sem problemas.
- **Sistema de Abas Inovador:** Inspirado pelo Essence OS, NexOS permite ao usuário gerenciar aplicações como abas dentro de janelas criadas pelo usuário, com a capacidade de abrir múltiplas aplicações dentro de uma única janela e destacá-las conforme necessário.
- **Simplicidade e Eficiência:** Ao eliminar a necessidade de um sistema de init tradicional e reduzir as peças móveis, NexOS oferece um boot quase instantâneo e uma experiência de desktop simplificada.
- **Desenvolvido em Rust:** A escolha de Rust garante segurança de memória e um desempenho elevado, além de facilitar a manutenção e a evolução do código.

## Estado Atual do Projeto

NexOS ainda está em desenvolvimento ativo. As principais funcionalidades, como o gerenciamento de processos, compatibilidade com programas Linux, e o compositor Wayland integrado, estão sendo implementadas. Este projeto é um trabalho em progresso e feedback construtivo é sempre bem-vindo.

## Executar uma VM

### Requisitos

- Xorriso
- Git
- Build Essential (gcc, make, etc...)
- QEMU (x86 ou aarch64)
- [rustup](https://rustup.rs) **(Recomendado)** | Compilador **nightly** do Rust
- Um computador com **Linux** (Se estiver a usar Windows, será necessário WSL), talvez "self-host" seja possivel no futuro (desenvolver o NexOS dentro do NexOS)

### Usar o Dev Shell do Nix

Pode também utilizar o [Nix](https://nixos.org) para configurar automaticamente o ambiente de desenvolvimento executando o comando `nix develop`.

### Passos

1. Clone o projeto:

```bash
git clone https://github.com/coffee-is-power/nexos
```

1. Compile o kernel e execute o QEMU:

```bash
cargo run
```

Pode também especificar um alvo diferente para compilar para outras arquiteturas e executar VMs em outras arquiteturas. A arquitetura principal é `x86_64`, mas o suporte a ARM também está a ser desenvolvido conforme vou aprendendo mais sobre.
