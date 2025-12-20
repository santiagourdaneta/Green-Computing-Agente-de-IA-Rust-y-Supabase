#!/bin/bash

echo "🛡️  Iniciando Quality Gate local..."

# 1. Verificar Formato
echo "📏 Paso 1: Revisando formato (rustfmt)..."
cargo fmt --all -- --check
if [ $? -ne 0 ]; then
    echo "❌ Error: Código mal formateado. Ejecuta 'cargo fmt' antes de subir."
    exit 1
fi

# 2. Verificar Linter
echo "🔍 Paso 2: Ejecutando Clippy (Linter Senior)..."
cargo clippy -- -D warnings
if [ $? -ne 0 ]; then
    echo "❌ Error: Clippy encontró problemas o warnings. Arréglalos antes de subir."
    exit 1
fi

echo "✅ Quality Gate aprobado. Procediendo con el commit..."
exit 0