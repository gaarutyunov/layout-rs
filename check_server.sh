#!/bin/bash
# Helper script to check if dev server is running

echo "Checking for running development server..."

# Check for trunk serve process
TRUNK_PID=$(pgrep -f "trunk serve")
if [ ! -z "$TRUNK_PID" ]; then
    echo "✅ Development server is already running (PID: $TRUNK_PID)"
    echo "   Access the application at: http://localhost:8080"
    exit 0
fi

# Check if port 8080 is in use
if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo "⚠️  Port 8080 is in use by another process"
    lsof -Pi :8080 -sTCP:LISTEN
    exit 1
fi

echo "❌ No development server running"
echo "   You can start it with: trunk serve --port 8080"
exit 2
