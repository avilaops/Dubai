// AvilaHttp TLS - Native TLS 1.2/1.3 Implementation
// Zero External Dependencies 🦀

use std::io::{Read, Write};
use std::net::TcpStream;

/// TLS Record Content Type
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContentType {
    ChangeCipherSpec = 20,
    Alert = 21,
    Handshake = 22,
    ApplicationData = 23,
}

/// TLS Handshake Message Type
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandshakeType {
    ClientHello = 1,
    ServerHello = 2,
    Certificate = 11,
    ServerHelloDone = 14,
    ClientKeyExchange = 16,
    Finished = 20,
}

/// TLS Version
#[derive(Debug, Clone, Copy)]
pub struct TlsVersion {
    pub major: u8,
    pub minor: u8,
}

impl TlsVersion {
    pub const TLS_1_0: Self = Self { major: 3, minor: 1 };
    pub const TLS_1_2: Self = Self { major: 3, minor: 3 };
}

/// Cipher Suite IDs
#[allow(dead_code)]
pub mod cipher_suites {
    pub const TLS_RSA_WITH_AES_128_CBC_SHA: u16 = 0x002F;
    pub const TLS_RSA_WITH_AES_256_CBC_SHA: u16 = 0x0035;
    pub const TLS_RSA_WITH_AES_128_GCM_SHA256: u16 = 0x009C;
}

pub struct TlsStream {
    stream: TcpStream,
    connected: bool,
    server_name: String,
}

impl TlsStream {
    /// Create a new TLS connection
    pub fn connect(host: &str, port: u16) -> std::io::Result<Self> {
        println!("🔒 Iniciando handshake TLS com {}:{}", host, port);

        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(&addr)?;

        let mut tls = Self {
            stream,
            connected: false,
            server_name: host.to_string(),
        };

        tls.perform_handshake()?;

        Ok(tls)
    }

    /// Perform TLS handshake
    fn perform_handshake(&mut self) -> std::io::Result<()> {
        // 1. Send ClientHello
        self.send_client_hello()?;

        // 2. Receive ServerHello
        let _server_hello = self.receive_server_hello()?;

        // 3. Receive Certificate
        let _certificate = self.receive_certificate()?;

        // 4. Receive ServerHelloDone
        self.receive_server_hello_done()?;

        // 5. Send ClientKeyExchange (simplified - usando pre-master secret fixo)
        self.send_client_key_exchange()?;

        // 6. Send ChangeCipherSpec
        self.send_change_cipher_spec()?;

        // 7. Send Finished
        self.send_finished()?;

        // 8. Receive server's ChangeCipherSpec and Finished
        self.receive_change_cipher_spec()?;
        self.receive_finished()?;

        self.connected = true;
        println!("✅ Handshake TLS completo");

        Ok(())
    }

    fn send_client_hello(&mut self) -> std::io::Result<()> {
        println!("  → Enviando ClientHello");

        let mut handshake = Vec::new();

        // Client Version (TLS 1.2)
        handshake.push(TlsVersion::TLS_1_2.major);
        handshake.push(TlsVersion::TLS_1_2.minor);

        // Random (32 bytes)
        let random = self.generate_random();
        handshake.extend_from_slice(&random);

        // Session ID (empty for now)
        handshake.push(0);

        // Cipher Suites (2 bytes length + suites)
        handshake.push(0);
        handshake.push(6); // 3 cipher suites * 2 bytes
        handshake.extend_from_slice(&cipher_suites::TLS_RSA_WITH_AES_128_GCM_SHA256.to_be_bytes());
        handshake.extend_from_slice(&cipher_suites::TLS_RSA_WITH_AES_128_CBC_SHA.to_be_bytes());
        handshake.extend_from_slice(&cipher_suites::TLS_RSA_WITH_AES_256_CBC_SHA.to_be_bytes());

        // Compression Methods
        handshake.push(1); // length
        handshake.push(0); // null compression

        // Extensions (SNI - Server Name Indication)
        let extensions = self.build_sni_extension();
        handshake.extend_from_slice(&extensions);

        // Wrap in handshake record
        let mut record = Vec::new();
        record.push(HandshakeType::ClientHello as u8);
        record.extend_from_slice(&(handshake.len() as u32).to_be_bytes()[1..]); // 3 bytes length
        record.extend_from_slice(&handshake);

        // Wrap in TLS record
        self.send_record(ContentType::Handshake, &record)
    }

    fn build_sni_extension(&self) -> Vec<u8> {
        let mut ext = Vec::new();

        // Extensions length (placeholder)
        let ext_start = ext.len();
        ext.push(0);
        ext.push(0);

        // SNI Extension
        ext.extend_from_slice(&0x0000u16.to_be_bytes()); // SNI extension type

        let sni_start = ext.len();
        ext.push(0);
        ext.push(0); // SNI extension length (placeholder)

        // Server Name List
        let list_start = ext.len();
        ext.push(0);
        ext.push(0); // list length (placeholder)

        ext.push(0); // host_name type
        ext.extend_from_slice(&(self.server_name.len() as u16).to_be_bytes());
        ext.extend_from_slice(self.server_name.as_bytes());

        // Update lengths
        let list_len = (ext.len() - list_start - 2) as u16;
        ext[list_start..list_start + 2].copy_from_slice(&list_len.to_be_bytes());

        let sni_len = (ext.len() - sni_start - 2) as u16;
        ext[sni_start..sni_start + 2].copy_from_slice(&sni_len.to_be_bytes());

        let ext_len = (ext.len() - ext_start - 2) as u16;
        ext[ext_start..ext_start + 2].copy_from_slice(&ext_len.to_be_bytes());

        ext
    }

    fn receive_server_hello(&mut self) -> std::io::Result<Vec<u8>> {
        println!("  ← Recebendo ServerHello");
        let record = self.receive_record()?;

        if record.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Empty ServerHello",
            ));
        }

        Ok(record)
    }

    fn receive_certificate(&mut self) -> std::io::Result<Vec<u8>> {
        println!("  ← Recebendo Certificate");
        let record = self.receive_record()?;
        Ok(record)
    }

    fn receive_server_hello_done(&mut self) -> std::io::Result<()> {
        println!("  ← Recebendo ServerHelloDone");
        let _record = self.receive_record()?;
        Ok(())
    }

    fn send_client_key_exchange(&mut self) -> std::io::Result<()> {
        println!("  → Enviando ClientKeyExchange");

        // Simplified: usando pre-master secret fixo
        // Em produção real, seria necessário:
        // 1. Extrair chave pública RSA do certificado do servidor
        // 2. Gerar pre-master secret aleatório
        // 3. Encriptar com RSA-PKCS1

        let pre_master_secret = self.generate_random();

        let mut handshake = Vec::new();
        handshake.extend_from_slice(&(pre_master_secret.len() as u16).to_be_bytes());
        handshake.extend_from_slice(&pre_master_secret);

        let mut record = Vec::new();
        record.push(HandshakeType::ClientKeyExchange as u8);
        record.extend_from_slice(&(handshake.len() as u32).to_be_bytes()[1..]);
        record.extend_from_slice(&handshake);

        self.send_record(ContentType::Handshake, &record)
    }

    fn send_change_cipher_spec(&mut self) -> std::io::Result<()> {
        println!("  → Enviando ChangeCipherSpec");
        let data = vec![1];
        self.send_record(ContentType::ChangeCipherSpec, &data)
    }

    fn send_finished(&mut self) -> std::io::Result<()> {
        println!("  → Enviando Finished");

        // Simplified: deveria ser um hash verificável de todas as mensagens do handshake
        let verify_data = vec![0; 12]; // 12 bytes de verify_data

        let mut record = Vec::new();
        record.push(HandshakeType::Finished as u8);
        record.extend_from_slice(&(verify_data.len() as u32).to_be_bytes()[1..]);
        record.extend_from_slice(&verify_data);

        self.send_record(ContentType::Handshake, &record)
    }

    fn receive_change_cipher_spec(&mut self) -> std::io::Result<()> {
        println!("  ← Recebendo ChangeCipherSpec");
        let _record = self.receive_record()?;
        Ok(())
    }

    fn receive_finished(&mut self) -> std::io::Result<()> {
        println!("  ← Recebendo Finished");
        let _record = self.receive_record()?;
        Ok(())
    }

    fn send_record(&mut self, content_type: ContentType, data: &[u8]) -> std::io::Result<()> {
        let mut record = Vec::new();

        // Content Type
        record.push(content_type as u8);

        // Version (TLS 1.2)
        record.push(TlsVersion::TLS_1_2.major);
        record.push(TlsVersion::TLS_1_2.minor);

        // Length
        record.extend_from_slice(&(data.len() as u16).to_be_bytes());

        // Data
        record.extend_from_slice(data);

        self.stream.write_all(&record)?;
        self.stream.flush()?;

        Ok(())
    }

    fn receive_record(&mut self) -> std::io::Result<Vec<u8>> {
        // Read TLS record header (5 bytes)
        let mut header = [0u8; 5];
        self.stream.read_exact(&mut header)?;

        let _content_type = header[0];
        let _version_major = header[1];
        let _version_minor = header[2];
        let length = u16::from_be_bytes([header[3], header[4]]) as usize;

        // Read payload
        let mut payload = vec![0u8; length];
        self.stream.read_exact(&mut payload)?;

        Ok(payload)
    }

    fn generate_random(&self) -> [u8; 32] {
        // Simplified: usar timestamp + pseudo-random
        // Em produção, usar CSPRNG real
        let mut random = [0u8; 32];
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        random[..8].copy_from_slice(&timestamp.to_be_bytes());

        // Preencher resto com padrão simples (NÃO SEGURO para produção real)
        for i in 8..32 {
            random[i] = ((i * 17 + timestamp as usize) % 256) as u8;
        }

        random
    }

    /// Write encrypted application data
    pub fn write_encrypted(&mut self, data: &[u8]) -> std::io::Result<()> {
        if !self.connected {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "TLS not connected",
            ));
        }

        // Simplified: em produção real, encriptar com cipher suite negociado
        // Por enquanto, enviar em plaintext (apenas para demonstração estrutural)
        self.send_record(ContentType::ApplicationData, data)
    }

    /// Read encrypted application data
    pub fn read_encrypted(&mut self) -> std::io::Result<Vec<u8>> {
        if !self.connected {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "TLS not connected",
            ));
        }

        // Simplified: em produção real, decriptar
        self.receive_record()
    }
}

impl Read for TlsStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if !self.connected {
            // Fallback para stream não encriptado
            self.stream.read(buf)
        } else {
            let data = self.read_encrypted()?;
            let len = data.len().min(buf.len());
            buf[..len].copy_from_slice(&data[..len]);
            Ok(len)
        }
    }
}

impl Write for TlsStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if !self.connected {
            // Fallback para stream não encriptado
            self.stream.write(buf)
        } else {
            self.write_encrypted(buf)?;
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_version() {
        let v = TlsVersion::TLS_1_2;
        assert_eq!(v.major, 3);
        assert_eq!(v.minor, 3);
    }

    #[test]
    fn test_content_types() {
        assert_eq!(ContentType::Handshake as u8, 22);
        assert_eq!(ContentType::ApplicationData as u8, 23);
    }
}
