#!/bin/bash

# Rust Architecture Visualizer - Git Setup Script
echo "🤖 Setting up Git for Rust Architecture Visualizer..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the rust-architecture-visualizer directory"
    exit 1
fi

# Initialize Git if not already initialized
if [ ! -d ".git" ]; then
    echo "📁 Initializing Git repository..."
    git init
else
    echo "✅ Git repository already initialized"
fi

# Add all files
echo "📝 Adding files to Git..."
git add .

# Check if there are changes to commit
if git diff --staged --quiet; then
    echo "ℹ️  No changes to commit"
else
    echo "💾 Committing changes..."
    git commit -m "Initial commit: Rust Architecture Visualizer

- Complete architecture visualizer for Rust projects
- Web interface with real-time updates  
- SVG rendering and HTML generation
- Dependency analysis and metrics calculation
- Configurable scanning and visualization options
- Default port set to 8000
- Ready for GitHub deployment"
fi

# Check if remote origin exists
if git remote get-url origin >/dev/null 2>&1; then
    echo "✅ Remote origin already configured"
    echo "🌐 Current remote: $(git remote get-url origin)"
else
    echo "🔗 Setting up GitHub remote..."
    echo "Please run the following commands to connect to GitHub:"
    echo ""
    echo "1. Create a new repository on GitHub:"
    echo "   - Go to https://github.com/new"
    echo "   - Repository name: rust-architecture-visualizer"
    echo "   - Description: A beautiful, real-time architecture visualizer for Rust projects"
    echo "   - Make it Public"
    echo "   - Don't initialize with README"
    echo ""
    echo "2. Connect your local repository:"
    echo "   git remote add origin https://github.com/harrync/rust-architecture-visualizer.git"
    echo "   git branch -M main"
    echo "   git push -u origin main"
    echo ""
fi

echo "✅ Git setup complete!"
echo ""
echo "🚀 Next steps:"
echo "1. Create GitHub repository (if not done already)"
echo "2. Run: git remote add origin https://github.com/harrync/rust-architecture-visualizer.git"
echo "3. Run: git push -u origin main"
echo ""
echo "🎯 To run the visualizer on your backend:"
echo "cargo run -- watch --project /Users/harrync/repos/titan-agent-backend"
