import os
from datetime import datetime

def decrypt_xor(encrypted_bytes):
    """Decrypt XOR-encrypted data with 0xAA key"""
    return bytes(b ^ 0xAA for b in encrypted_bytes)

def clean_emoji(text):
    """Fix corrupted emoji characters"""
    return text.replace("ðŸ–±ï¸", "🖱️")

def format_log_entry(entry):
    """Format a single log entry with proper spacing and emojis"""
    entry = clean_emoji(entry.strip())
    if not entry:
        return ""
    
    # Add newlines before important events
    if "SESSION START" in entry:
        return f"\n{'='*50}\n{entry}\n{'='*50}\n"
    elif "Key Pressed" in entry or "Key Released" in entry:
        return f"\n⌨️ {entry}"
    elif "Mouse Click" in entry:
        return f"\n🖱️ {entry}"
    return entry  # Mouse movements will stay on single lines

def process_log_file(input_path, output_path):
    """Process and format the entire log file"""
    try:
        with open(input_path, "rb") as f:
            encrypted_data = f.read()
        
        if not encrypted_data:
            print("⚠️ Warning: File is empty")
            return False
            
        decrypted = decrypt_xor(encrypted_data).decode('utf-8', errors='replace')
        
        # Split and process each line
        formatted_lines = []
        for line in decrypted.split('\n'):
            formatted = format_log_entry(line)
            if formatted:
                formatted_lines.append(formatted)
        
        # Write formatted output
        with open(output_path, "w", encoding="utf-8") as f:
            f.write("\n".join(formatted_lines))
        
        return True
        
    except Exception as e:
        print(f"❌ Error processing file: {str(e)}")
        return False

def main():
    print("🔓 CrossKey Log Formatter")
    print("="*50)
    input_path = input("Enter path to encrypted .bin file: ").strip('"\' ')
    
    if not os.path.exists(input_path):
        print(f"❌ Error: File not found at {input_path}")
        return
    
    output_path = os.path.splitext(input_path)[0] + "_formatted.log"
    
    if process_log_file(input_path, output_path):
        print(f"\n✅ Successfully formatted log saved to:\n{output_path}")
        
        # Show preview
        with open(output_path, "r", encoding="utf-8") as f:
            print("\nPreview of formatted log:")
            for _ in range(10):  # Show first 10 lines
                line = f.readline()
                if not line:
                    break
                print(line.strip())
            print("...")

if __name__ == "__main__":
    main()