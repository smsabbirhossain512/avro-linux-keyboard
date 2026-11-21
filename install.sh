#!/bin/bash
set -e

echo "🚀 Avro Phonetic (Rust Edition) ইনস্টলেশন শুরু হচ্ছে..."

# ১. ডিস্ট্রো অনুযায়ী প্রয়োজনীয় ডিপেন্ডেন্সি (IBus, Rust, GCC) ইনস্টল করা
if command -v apt &> /dev/null; then
    echo "📦 Ubuntu/Debian ডিটেক্ট হয়েছে। ডিপেন্ডেন্সি চেক করা হচ্ছে..."
    sudo apt update
    sudo apt install -y ibus libibus-1.0-dev build-essential curl pkg-config
elif command -v dnf &> /dev/null; then
    echo "📦 Fedora ডিটেক্ট হয়েছে। ডিপেন্ডেন্সি চেক করা হচ্ছে..."
    sudo dnf install -y ibus ibus-devel gcc curl pkgconf-pkg-config
elif command -v pacman &> /dev/null; then
    echo "📦 Arch Linux ডিটেক্ট হয়েছে। ডিপেন্ডেন্সি চেক করা হচ্ছে..."
    sudo pacman -S --noconfirm --needed ibus base-devel curl pkgconf
fi

# ২. Rust/Cargo না থাকলে অটোমেটিক ইনস্টল করা
if ! command -v cargo &> /dev/null; then
    echo "🦀 Rust (cargo) পাওয়া যায়নি, ইনস্টল করা হচ্ছে..."
    curl --proto '=https' --tlsv1.2 -sSf https://rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# ৩. প্রজেক্ট বিল্ড ও ইনস্টল করা
echo "⚙️ ইঞ্জিন কম্পাইল করা হচ্ছে (একটু সময় লাগতে পারে)..."
cargo build --release

echo "📂 সিস্টেমে ফাইল কপি করা হচ্ছে..."
sudo install -Dm755 target/release/avro-rust /usr/local/bin/avro-rust

# XML কম্পোনেন্ট ফাইল তৈরি (যদি আপনার স্ক্রিপ্টে আলাদা XML থাকে তবে সেটি কপি করতে পারেন)
sudo mkdir -p /usr/share/ibus/component
cat << 'EOF' | sudo tee /usr/share/ibus/component/avro-rust.xml > /dev/null
<?xml version="1.0" encoding="utf-8"?>
<component>
    <name>org.freedesktop.IBus.AvroRust</name>
    <description>Avro Phonetic (Rust Ultra-Fast)</description>
    <exec>/usr/local/bin/avro-rust --ibus</exec>
    <version>2.0.0</version>
    <author>SM Sabbir Hossain</author>
    <license>GPL</license>
    <homepage>https://github.com</homepage>
    <textdomain>avro-rust</textdomain>
    <engines>
        <engine>
            <name>avro-rust</name>
            <language>bn</language>
            <license>GPL</license>
            <author>SM Sabbir Hossain</author>
            <layout>us</layout>
            <longname>Avro Phonetic (Rust Ultra-Fast)</longname>
            <description>Avro Phonetic Bangla Input Method in Rust</description>
            <rank>99</rank>
        </engine>
    </engines>
</component>
EOF

# ৪. Ubuntu 24.04/26.04 Wayland-এর IBus সমস্যা ফিক্স ও অটোস্টার্ট সেট করা
echo "🔧 IBus কনফিগার ও চালু করা হচ্ছে..."
gsettings set org.gnome.desktop.interface gtk-im-module 'ibus' 2>/dev/null || true

if ! pgrep -x "ibus-daemon" > /dev/null; then
    ibus-daemon -drx
    sleep 2
fi

mkdir -p "$HOME/.config/autostart"
cat << 'EOF' > "$HOME/.config/autostart/ibus-daemon.desktop"
[Desktop Entry]
Type=Application
Name=IBus Daemon
Exec=ibus-daemon -drx
X-GNOME-Autostart-enabled=true
EOF

sudo ibus write-cache
ibus restart

# ৫. GNOME Input Sources-এ অটোমেটিক যুক্ত করা
if command -v gsettings &> /dev/null; then
    CURRENT_SOURCES=$(gsettings get org.gnome.desktop.input-sources sources)
    if [[ "$CURRENT_SOURCES" != *"avro-rust"* ]]; then
        if [[ "$CURRENT_SOURCES" == "@a(ss) []" ]]; then
            NEW_SOURCES="[('xkb', 'us'), ('ibus', 'avro-rust')]"
        else
            NEW_SOURCES="${CURRENT_SOURCES%]*}, ('ibus', 'avro-rust')]"
        fi
        gsettings set org.gnome.desktop.input-sources sources "$NEW_SOURCES"
    fi
fi

echo "✅ ইনস্টলেশন সফল হয়েছে! একবার Log Out করে Log In করুন এবং Super + Space চেপে বাংলা লিখুন।"

