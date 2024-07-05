# Utilisation d'une image Debian stable comme base
FROM debian:stable

# Mise à jour des paquets et installation des outils nécessaires
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Installation de Rust via rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Ajout de ~/.cargo/bin au PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Définition du répertoire de travail dans le conteneur
WORKDIR /app

# Copie des fichiers source dans le répertoire de travail
COPY . .

# Commande par défaut à exécuter lorsque le conteneur démarre
CMD ["bash"]
