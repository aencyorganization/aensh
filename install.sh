#!/bin/bash

# Aensh Shell Installation Script
# This script builds and installs the Aensh shell
# Usage: curl -sSL https://raw.githubusercontent.com/aencyorganization/aensh/main/install.sh | bash

set -e

REPO_URL="https://github.com/aencyorganization/aensh"
INSTALL_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/aensh"

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║           🦀 Aensh Shell Installer 🦀                        ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Function to add to PATH
add_to_path() {
    local shell_rc="$1"
    local path_line='export PATH="$HOME/.local/bin:$PATH"'
    
    if [ -f "$shell_rc" ]; then
        if ! grep -q '.local/bin' "$shell_rc" 2>/dev/null; then
            echo "" >> "$shell_rc"
            echo "# Added by Aensh installer" >> "$shell_rc"
            echo "$path_line" >> "$shell_rc"
            echo "✓ PATH adicionado em $shell_rc"
        fi
    fi
}

# Check and install Rust if needed
install_rust() {
    if command -v cargo &> /dev/null; then
        echo "✓ Rust/Cargo já instalado: $(cargo --version)"
        return 0
    fi
    
    echo "📦 Instalando Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Source cargo env
    source "$HOME/.cargo/env" 2>/dev/null || true
    
    if ! command -v cargo &> /dev/null; then
        echo "❌ Falha ao instalar Rust. Por favor, instale manualmente:"
        echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    echo "✓ Rust instalado com sucesso!"
}

# Clone or update repository
get_source() {
    local temp_dir="/tmp/aensh-install-$$"
    
    if [ -d ".git" ] && [ -f "Cargo.toml" ] && grep -q "aensh" Cargo.toml 2>/dev/null; then
        echo "✓ Usando diretório atual como fonte"
        return 0
    fi
    
    echo "📥 Baixando Aensh..."
    
    if command -v git &> /dev/null; then
        rm -rf "$temp_dir"
        git clone --depth 1 "$REPO_URL" "$temp_dir" 2>/dev/null || {
            echo "⚠️  Git clone falhou, tentando download direto..."
            download_source "$temp_dir"
        }
    else
        download_source "$temp_dir"
    fi
    
    cd "$temp_dir"
}

download_source() {
    local dest="$1"
    mkdir -p "$dest"
    curl -sL "$REPO_URL/archive/main.tar.gz" | tar xz -C "$dest" --strip-components=1
}

# Build Aensh
build_aensh() {
    echo "🔨 Compilando Aensh..."
    cargo build --release
    
    if [ ! -f "target/release/aensh" ]; then
        echo "❌ Falha na compilação"
        exit 1
    fi
    
    echo "✓ Compilação concluída!"
}

# Install binary
install_binary() {
    echo "📦 Instalando..."
    
    mkdir -p "$INSTALL_DIR"
    
    # Check if aensh is running and kill it
    if [ -f "$INSTALL_DIR/aensh" ]; then
        # Try to remove the old binary, if busy, rename it first
        if ! rm -f "$INSTALL_DIR/aensh" 2>/dev/null; then
            echo "⚠️  Binário em uso, substituindo..."
            mv "$INSTALL_DIR/aensh" "$INSTALL_DIR/aensh.old" 2>/dev/null || true
            rm -f "$INSTALL_DIR/aensh.old" 2>/dev/null || true
        fi
    fi
    
    cp target/release/aensh "$INSTALL_DIR/aensh"
    chmod +x "$INSTALL_DIR/aensh"
    
    echo "✓ Binário instalado em $INSTALL_DIR/aensh"
}

# Setup config directories
setup_config() {
    mkdir -p "$CONFIG_DIR/plugins"
    
    # Create default config if not exists
    if [ ! -f "$CONFIG_DIR/config.json" ]; then
        echo '{"default_shell": false}' > "$CONFIG_DIR/config.json"
    fi
    
    echo "✓ Diretórios de configuração criados"
}

# Setup PATH
setup_path() {
    echo "🔧 Configurando PATH..."
    
    # Add to various shell configs
    add_to_path "$HOME/.bashrc"
    add_to_path "$HOME/.zshrc"
    add_to_path "$HOME/.profile"
    
    # Export for current session
    export PATH="$INSTALL_DIR:$PATH"
}

# Main installation
main() {
    echo "Passo 1/5: Verificando Rust..."
    install_rust
    
    echo ""
    echo "Passo 2/5: Obtendo código fonte..."
    get_source
    
    echo ""
    echo "Passo 3/5: Compilando..."
    build_aensh
    
    echo ""
    echo "Passo 4/5: Instalando..."
    install_binary
    setup_config
    
    echo ""
    echo "Passo 5/5: Configurando PATH..."
    setup_path
    
    echo ""
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║           ✅ Instalação Concluída!                           ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo ""
    echo "🚀 Para iniciar o Aensh:"
    echo "   aensh"
    echo ""
    echo "📚 Para ver a ajuda:"
    echo "   aensh --help"
    echo ""
    echo "⚙️  Para definir como shell padrão:"
    echo "   aensh --default true"
    echo ""
    echo "🔌 Diretório de plugins:"
    echo "   $CONFIG_DIR/plugins/"
    echo ""
    echo "⚠️  Se 'aensh' não for encontrado, reinicie o terminal ou execute:"
    echo "   source ~/.bashrc  # ou ~/.zshrc"
    echo ""
    echo "🗑️  Para desinstalar:"
    echo "   rm -f $INSTALL_DIR/aensh"
    echo "   rm -rf $CONFIG_DIR"
    echo ""
}

main "$@"
