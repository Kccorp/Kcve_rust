# KcVE_Rust
KcVE is a comprehensive CVE (Common Vulnerabilities and Exposures) tool write in rust that allows users to gather and parse information about the latest vulnerabilities from publicly available sources. It features are easy-to-use that allows users to access informations with just a few clicks.

The app exports the information in an HTML format, making it easy to share and analyze. This tool is ideal for security professionals, researchers, and others who need to stay informed about the latest vulnerabilities and ensure the security of their systems.

With KcVE, users can access the latest information about CVEs quickly and efficiently, giving them the ability to take action to protect their systems and data.

## Usage

### Option 1: run from source

```bash
cargo run -- [-h] -s STARTDATE -e ENDDATE [-k KEYWORD]
```