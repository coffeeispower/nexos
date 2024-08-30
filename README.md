# NexOS

Bem-vindo ao **NexOS**, um sistema operativo moderno e pensado para o futuro. Criado para tornar a tua experiência com computadores mais simples e eficiente, o NexOS é perfeito para quem quer um sistema rápido, seguro e fácil de usar.

## Por que escolher o NexOS?

- **Desempenho Rápido:** NexOS arranca em poucos segundos e faz tudo o que precisas sem te deixar à espera.
- **Interface Simples:** Navegar entre as tuas aplicações nunca foi tão fácil. Podes abrir várias janelas e organizar tudo como preferires.
- **Compatível com Aplicações Linux:** Se usas programas Linux, podes continuar a utilizá-los aqui sem problemas. Até as aplicações mais antigas são suportadas!
- **Seguro e Confiável:** Desenvolvido em [Rust](https://www.rust-lang.org/), uma linguagem conhecida pela segurança, o NexOS garante que os teus dados estão protegidos.

## Como está o projeto?

O NexOS ainda está em desenvolvimento, mas já tem várias funcionalidades chave em funcionamento. Estamos sempre a melhorar e adicionar novas funcionalidades. O teu feedback é importante para nós!

## Como testar o NexOS?

Se quiseres experimentar o NexOS numa máquina virtual, segue estes passos simples:

### O que precisas:

- **Um computador com Linux:** Para Windows, precisarás do WSL (Windows Subsystem for Linux).
- **Programas necessários:** Xorriso, Git, Build Essential (gcc, make, etc.), QEMU (x86 ou aarch64), e [rustup](https://rustup.rs) com o compilador **nightly** de Rust.
- **Nix (opcional):** Podes usar o [Nix](https://nixos.org) para configurar automaticamente o ambiente de desenvolvimento.

### Passos:

1. Clonar o projeto:

    ```bash
    git clone https://github.com/coffee-is-power/nexos
    ```

2. Compilar o kernel e executar o QEMU:

    ```bash
    cargo run
    ```

Podes também compilar para outras arquiteturas, como ARM, se preferires.
