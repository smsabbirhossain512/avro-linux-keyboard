#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE}")" && pwd)"

if [ ! -f "$SCRIPT_DIR/target/release/avro-rust" ]; then
    cargo build --release
fi

sudo cp "$SCRIPT_DIR/target/release/avro-rust" /usr/local/bin/avro-rust
sudo chmod +x /usr/local/bin/avro-rust

sudo mkdir -p /usr/share/ibus/component
cat << 'XML' | sudo tee /usr/share/ibus/component/avro-rust.xml > /dev/null
<?xml version="1.0" encoding="utf-8"?>
<component>
    <name>org.freedesktop.IBus.AvroRust</name>
    <description>Rust Avro Phonetic Engine</description>
    <exec>/usr/local/bin/avro-rust --ibus</exec>
    <version>1.0</version>
    <author>Sabbir</author>
    <license>GPL</license>
    <homepage>https://github.com</homepage>
    <textdomain>avro-rust</textdomain>
    <engines>
        <engine>
            <name>avro-rust</name>
            <language>bn</language>
            <license>GPL</license>
            <author>Sabbir</author>
            <layout>us</layout>
            <longname>Avro Phonetic (Rust Ultra-Fast)</longname>
            <description>High-Speed Rust Bangla Phonetic Keyboard</description>
            <rank>99</rank>
        </engine>
    </engines>
</component>
XML

ibus restart
echo "✅ Rust Avro Keyboard সফলভাবে ইনস্টল হয়েছে!"
