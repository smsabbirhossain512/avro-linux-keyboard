#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Installing dependencies..."
if command -v dnf &> /dev/null; then
    sudo dnf install -y ibus python3-gobject google-noto-sans-bengali-fonts
elif command -v apt &> /dev/null; then
    sudo apt update && sudo apt install -y ibus python3-gi gir1.2-ibus-1.0 fonts-noto-core
elif command -v pacman &> /dev/null; then
    sudo pacman -S --noconfirm ibus python-gobject noto-fonts
fi

sudo mkdir -p /usr/share/avro-fedora /usr/share/ibus/component
sudo cp "$SCRIPT_DIR/avro_engine.py" /usr/share/avro-fedora/
sudo chmod +x /usr/share/avro-fedora/avro_engine.py

cat << 'XML' | sudo tee /usr/share/ibus/component/avro-fedora.xml > /dev/null
<?xml version="1.0" encoding="utf-8"?>
<component>
    <name>org.freedesktop.IBus.AvroFedora</name>
    <description>Universal Linux Avro Phonetic Engine</description>
    <exec>/usr/bin/python3 /usr/share/avro-fedora/avro_engine.py --ibus</exec>
    <version>1.0</version>
    <author>Sabbir</author>
    <license>GPL</license>
    <homepage>https://localhost</homepage>
    <textdomain>avro-fedora</textdomain>
    <engines>
        <engine>
            <name>avro-fedora</name>
            <language>bn</language>
            <license>GPL</license>
            <author>Sabbir</author>
            <layout>us</layout>
            <longname>Avro Phonetic (Custom)</longname>
            <description>Bangla Phonetic Keyboard for Linux</description>
            <rank>99</rank>
        </engine>
    </engines>
</component>
XML

ibus restart
echo "✅ ইনস্টলেশন সফল! এখন Settings > Keyboard থেকে 'Avro Phonetic (Custom)' যুক্ত করুন অথবা একবার Log Out করে Log In করুন।"
