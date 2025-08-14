# Rust nightly image'ini kullanın. Bu image, Rust'ı build etmek için gerekli tüm araçları içerir.
FROM rustlang/rust:nightly as builder

# Çalışma dizinini ayarlayın
WORKDIR /usr/src/rustmicro

# Cargo.lock ve Cargo.toml dosyalarını kopyalayın. Bu, bağımlılıkların yüklenmesini hızlandırır.
# Docker, bu dosyalar değişmediği sürece bu katmanı cache'ler.
COPY Cargo.toml Cargo.lock ./

# Dummy bir main.rs dosyası oluşturun. Bu, bağımlılıkların yüklenmesini sağlar.
# Gerçek kod daha sonra kopyalanacak, bu yüzden bağımlılıkların yüklenmesi için bir kerelik bir işlem.
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

# Bağımlılıkları derleyin. Bu adım, bağımlılıklar değişmediği sürece tekrar çalıştırılmaz.
# Bu, build süresini önemli ölçüde azaltır.
RUN cargo build --release --locked
RUN rm -f target/release/deps/rustmicro*

# Şimdi gerçek kaynak kodu kopyalayın.
COPY . .

# Uygulamayı build edin.
# --offline flag'i kaldırıldı çünkü bazı bağımlılıklar için internet bağlantısı gerekebilir.
RUN cargo build --release --locked

# Runtime image
# Daha yeni bir Debian sürümü kullanarak GLIBC uyumsuzluğunu çözün.
FROM debian:bookworm-slim

# libpq kütüphanesini kurun. Bu, PostgreSQL client'ı için gereklidir.
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Builder stage'inden uygulamayı kopyalayın.
COPY --from=builder /usr/src/rustmicro/target/release/rustmicro /usr/local/bin/rustmicro

# Varsayılan portu expose edin.
EXPOSE 8080

# Uygulamayı çalıştırın.
CMD ["rustmicro"]