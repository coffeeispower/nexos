# NexOS

O **NexOS** é um sistema operativo moderno e inovador, criado para repensar a experiência do utilizador e a arquitetura tradicional de sistemas baseados em Unix.
Desenvolvido do zero em [Rust](https://www.rust-lang.org/), o NexOS combina a robustez e a segurança da linguagem com uma abordagem minimalista e eficiente, para criar um sistema operativo desktop poderoso e fácil de se utilizar.

O **NexOS** oferece uma experiência de sistema operativo moderno e rápido, sem a complexidade desnecessária das distribuições tradicionais de Linux. Com foco na simplicidade, desempenho e usabilidade, o NexOS pretende ser o "Linux como seria se fosse projetado em 2024".

## Características Principais

- **Compositor Wayland Integrado:** O **NexOS** incorpora diretamente um compositor [Wayland](https://wayland.freedesktop.org/) no Kernel, oferecendo um ambiente gráfico rápido e responsivo sem a necessidade de camadas adicionais ou comunicação entre processos (IPC).
- **100% Compatível com Binários de Linux:** O **NexOS** é totalmente compatível com os binários de Linux, permitindo que qualquer programa que utilize Wayland para a GUI funcione perfeitamente. Além disso, o suporte ao XWayland garante que aplicações antigas ainda baseadas em Xorg possam ser executadas sem problemas.
- **Sistema de Separadores Inovador:** Inspirado no [Essence OS](https://nakst.gitlab.io/essence), o **NexOS** permite aos utilizadores uma forma inovadora de organizem as suas aplicações em separadores dentro de janelas criadas pelo utilizador. Cada janela criada pode conter várias aplicações abertas pelos seus respetivos separadores.
    > "As aplicações não são abertas em janelas próprias, ao invés disso, o utilizador cria as janelas e organiza as aplicações em separadores dentro dessa janela, semelhante a um navegador".
- **Simplicidade e Eficiência:** O **NexOS** reduz o tempo de arranque e simplifica a experiência de ambiente de trabalho, com a eliminação de um sistema de inicialização (Init) tradicional, o que reduz o número de componentes e pontos de falha.
- **Desenvolvido em Rust:** A escolha da linguagem de programação **[Rust](https://www.rust-lang.org/)** garante a segurança de memória e um desempenho elevado, bem como, facilitar a manutenção e a evolução do código.

## Estado Atual do Projeto

O **NexOS** ainda está em desenvolvimento ativo. As principais funcionalidades, como a gestão de processos, compatibilidade com programas de Linux, e o compositor Wayland integrado, estão a ser implementadas. Este projeto é um trabalho em progresso e feedback construtivo é sempre bem-vindo.

## Executar em uma VM

### Requisitos

- Xorriso
- Git
- Build Essential (gcc, make, etc...)
- QEMU (x86 ou aarch64)
- [rustup](https://rustup.rs) **(Recomendado)** | Compilador **nightly** do Rust
- Um computador com **Linux** (Se estiver a utilizar Windows, será necessário WSL), talvez "self-host" seja possivel no futuro (desenvolver o NexOS dentro do NexOS)

### Utilizar o Dev Shell do Nix

Pode também utilizar o [Nix](https://nixos.org) para configurar automaticamente o ambiente de desenvolvimento executando o comando `nix develop`.

### Passos

1. Clone o projeto:

```bash
git clone https://github.com/coffee-is-power/nexos
```

1. Compile o Kernel e execute o QEMU:

```bash
cargo run
```

Pode também especificar um alvo diferente para compilar para outras arquiteturas e executar VMs em outras arquiteturas. A arquitetura principal é `x86_64`, mas o suporte a ARM também está a ser desenvolvido conforme vou aprendendo mais sobre.
