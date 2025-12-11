# 📂 Organizer CLI

Um organizador de arquivos **rápido**, **seguro** e **configurável** escrito em Rust.
Mova automaticamente seus arquivos para pastas categorizadas (Imagens, Documentos, Vídeos, etc.) com um único comando.

## 🚀 Funcionalidades

- **⚡ Zero Configuração Inicial**: Já vem com categorias padrão inteligentes.
- **⚙️ Totalmente Configurável**: Edite as regras e crie suas próprias categorias (`organizer config`).
- **🛡️ Seguro**: Nunca sobrescreve arquivos. Se houver duplicatas, ele renomeia automaticamente (`foto.jpg` -> `foto_1.jpg`).
- **🚫 Ignora Ocultos**: Opção para ignorar arquivos de sistema/ocultos (`--ignore-hidden`).
- **🐧💻 Cross-Platform**: Funciona perfeitamente no Windows, Linux e macOS.

## 📦 Instalação

### Pré-requisitos
- Rust (Cargo) instalado.

```bash
# Clone o repositório
git clone https://github.com/seu-usuario/organizer.git
cd organizer

# Instale localmente
cargo install --path .
```

## 🛠️ Como Usar

### 1. Organizar a pasta atual
```bash
organizer
```

### 2. Organizar uma pasta específica
```bash
organizer "C:\Users\Voce\Downloads"
```

### 3. Definir uma pasta de destino diferente
```bash
organizer ./Downloads --output ./Downloads/Organizados
```

### 4. Configurar Categorias
Quer mudar quais arquivos vão para onde?
```bash
organizer config
```
Isso abrirá o arquivo `config.toml` no seu editor padrão. Exemplo:

```toml
[rules]
Imagens = ["jpg", "png", "gif"]
ProjetosRust = ["rs", "toml"]
# Adicione suas próprias regras!
```

## 🤝 Contribuição
Sinta-se livre para abrir issues ou pull requests!
