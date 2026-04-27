# ForgeOS

ForgeOS est un système d’exploitation 64 bits personnalisé, développé entièrement from scratch en Rust, Assembly et C.

Le but de ce projet est de construire un véritable système bas niveau avec un fort accent sur :

* la performance
* la fiabilité
* l’architecture système
* la compréhension du matériel bas niveau
* le développement de kernel depuis zéro

ForgeOS commence dès le processus de démarrage BIOS et construit progressivement une chaîne complète de démarrage :

```text id="f7k2m1"
BIOS → Boot Sector → Bootloader → Protected Mode → Long Mode → Kernel Rust
```

Ce projet n’est basé ni sur Linux, ni sur Windows, ni sur un kernel existant.

Tout est développé manuellement afin de comprendre pleinement le fonctionnement interne d’un système d’exploitation.

---

## Objectifs du projet

### Objectifs actuels

* Boot sector BIOS personnalisé
* Bootloader personnalisé (Assembly + C)
* Transition vers le Protected Mode
* Transition vers le Long Mode (64 bits)
* Point d’entrée du kernel Rust en `no_std`
* Affichage texte VGA
* Gestion mémoire de base
* Gestion des interruptions (IDT)

### Objectifs futurs

* Système de paging
* Allocateur de heap
* Scheduler
* Gestion des processus
* Système de fichiers
* Drivers
* Programmes userland
* Syscalls
* Shell
* Réseau
* Interface graphique

---

## Structure du projet

```text id="b4m8zp"
ForgeOS/
│
├── boot/
│   ├── bios/          # Boot sector BIOS (16 bits)
│   └── loader/        # Bootloader (ASM + C)
│
├── kernel/            # Kernel Rust
│
├── build/             # Fichiers générés
│
├── Makefile           # Système de build
│
└── README.md
```

---

## Technologies utilisées

### Rust

Utilisé pour le kernel et la logique principale du système.

Pourquoi Rust ?

* sécurité mémoire
* abstractions sans coût de performance
* système de types robuste
* programmation système moderne
* excellent pour le développement de kernel

### Assembly (x86 / x86_64)

Utilisé pour :

* le boot sector
* les transitions de modes CPU
* la configuration de la GDT
* la configuration du paging
* l’accès matériel bas niveau

### C

Utilisé dans le bootloader lorsque certaines logiques bas niveau sont plus simples à gérer avant l’entrée dans le kernel Rust.

---

## Build & Exécution

### Dépendances nécessaires

Installer :

* `nasm`
* `gcc`
* `ld`
* `qemu`
* `make`
* `gdb`

Exemple (Linux) :

```bash id="6jz2ws"
sudo apt install nasm gcc qemu-system-x86 make
```

---

## Commandes

### Compiler le projet

```bash id="d3s9ak"
make
```

Cette commande compile :

* le boot sector BIOS
* le bootloader
* l’image disque bootable

---

### Lancer ForgeOS dans QEMU

```bash id="s1k8vn"
make run
```

Cette commande lance le système d’exploitation dans QEMU.

---

### Mode debug

```bash id="u9n4lt"
make debug
```

Cette commande lance QEMU avec des options de debug pour faciliter le diagnostic des erreurs.

---

### Nettoyer les fichiers générés

```bash id="r5m2xp"
make clean
```

Cette commande supprime les binaires, fichiers objets et images disque générés.

---

### En développement

ForgeOS est actuellement en développement actif.

---

## Licence

Ce projet est sous licence GNU GPL v3.

ForgeOS est open source et a vocation à le rester.

---

## Auteur

Créé par Gaétan Ledoux