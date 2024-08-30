# NexOS

**NexOS** é um sistema operativo de nova estirpe, forjado com o desígnio de repensar a experiência do utente e a estrutura dos sistemas baseados em Unix, tal como são conhecidos nos dias que correm. Forjado de raíz em [Rust](https://www.rust-lang.org/), NexOS conjuga a robustez e segurança desta linguagem com uma abordagem minimalista e eficiente, criando um ambiente optimizado para Desktop, sendo ao mesmo tempo poderoso e de fácil manejo.

Nosso intento com o NexOS é prover uma experiência que se sinta moderna e ágil, purgada da complexidade desnecessária que tantas vezes acompanha as distribuições Linux do tempo presente. Com atenção redobrada na simplicidade, desempenho e usabilidade, o NexOS almeja ser aquilo que o "Linux seria, caso fosse concebido no ano de 2024."

## Principais Carácteres

- **Compositor Wayland Integrado:** NexOS alberga em seu próprio âmago um compositor Wayland, o qual permite um ambiente gráfico rápido e pronto a responder, sem a necessidade de camadas adicionais ou comunicação entre processos (IPC).
- **Compatibilidade Total com Binários de Linux:** NexOS mostra-se plenamente compatível com binários de Linux, facultando que qualquer programa que utilize Wayland para a interface gráfica funcione sem peias. Ademais, o suporte a XWayland garante que aplicações vetustas, ainda baseadas em Xorg, possam correr sem contratempos.
- **Sistema de Abas Inovador:** Inspirado pelo Essence OS, NexOS concede ao utente a faculdade de manobrar aplicações como se fossem abas, dentro de janelas que o próprio pode criar, com a possibilidade de abrir múltiplas aplicações numa única janela e destacá-las conforme o desejo.
- **Simplicidade e Eficiência:** Ao banir a necessidade de um sistema de inicialização tradicional e ao reduzir os componentes móveis, NexOS oferece um arranque quase instantâneo e uma experiência de trabalho simplificada.
- **Desenvolvido em Rust:** A eleição de Rust como linguagem garante segurança na gestão da memória e desempenho elevado, além de facilitar a manutenção e a constante evolução do código.

## Estado Atual do Projecto

NexOS ainda se encontra em pleno desenvolvimento. As principais funcionalidades, tais como a gestão de processos, compatibilidade com programas Linux e o compositor Wayland integrado, estão a ser implementadas. Este projecto é uma obra em progresso, e todo o feedback construtivo será recebido com grande apreço.

## Executar uma Máquina Virtual

### Requisitos

- Xorriso
- Git
- Build Essential (gcc, make, etc.)
- QEMU (x86 ou aarch64)
- [rustup](https://rustup.rs) **(Recomendado)** | Compilador **nightly** de Rust
- Um computador provido de **Linux** (Se estiver a usar Windows, ser-lhe-á necessário o WSL), conquanto o "self-hosting" possa tornar-se exequível no porvir (desenvolver o NexOS dentro do próprio NexOS).

### Usar o Dev Shell do Nix

Poderá também recorrer ao [Nix](https://nixos.org) para configurar automaticamente o ambiente de desenvolvimento, executando o comando `nix develop`.

### Passos

1. Clonar o projecto:

    ```bash
    git clone https://github.com/coffee-is-power/nexos
    ```

2. Compilar o kernel e executar o QEMU:

    ```bash
    cargo run
    ```

Pode também especificar um alvo diferente para compilar para outras arquitecturas e executar máquinas virtuais em outros ambientes. A arquitectura principal é `x86_64`, mas o suporte a ARM está igualmente a ser desenvolvido, à medida que mais saber se adquire sobre tal empreitada.
