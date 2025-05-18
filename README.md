# LASER MULTIPLAYER

## Description
Laser Maze Wars est une réinterprétation moderne du jeu classique Maze Wars, transformé en jeu de laser tag multijoueur. Les joueurs s'affrontent dans un labyrinthe 3D où ils doivent éliminer leurs adversaires en les touchant avec leur laser. Chaque joueur peut être touché 3 fois avant d'être éliminé.

## Fonctionnalités
- Architecture client-serveur utilisant le protocole UDP
- Support pour 8+ joueurs simultanés (minimum 2 pour démarrer une partie)
- 3 niveaux de difficulté avec des labyrinthes de plus en plus complexes
- Mini-carte affichant la position du joueur et la structure du labyrinthe
- Affichage du nombre de FPS (toujours maintenu au-dessus de 50 FPS)
- Compatibilité avec manette de jeu et clavier/souris
- Compte à rebours de 10 secondes pour permettre aux joueurs supplémentaires de rejoindre la partie

## Prérequis
- Rust (édition 2021 ou plus récente)
- Cargo
- Connexion réseau pour le mode multijoueur

## Installation

### Cloner le dépôt
```bash
git clone https://github.com/votre-username/laser-maze-wars.git
cd laser-maze-wars
```

## Lancement du serveur
```bash
cd server
cargo run
```

Le serveur affichera son adresse IP lorsqu'il sera démarré.

## Lancement du client
```bash
cd client
cargo run
```

Lors du démarrage du client, vous devrez entrer:
1. L'adresse IP du serveur (format: xxx.xxx.xxx.xxx:port)
   - Pour jouer en local sur la même machine que le serveur, utilisez l'adresse locale affichée par le serveur
   - Pour rejoindre une partie distante, utilisez l'adresse IP du serveur distant
2. Votre nom d'utilisateur

## Contrôles

### Clavier/Souris
- **W** : Avancer
- **S** : Reculer
- **A** : Déplacement latéral gauche
- **D** : Déplacement latéral droit
- **Souris** : Orientation/visée
- **Clic gauche** : Tirer avec le laser

### Manette
- **Joystick gauche** : Déplacement
- **Joystick droit** : Orientation/visée
- **Gâchette droite (R2/RT)** : Tirer avec le laser

## Règles du jeu
1. Chaque joueur commence avec 3 points de vie
2. Un joueur perd un point de vie lorsqu'il est touché par le laser d'un adversaire
3. Un joueur est éliminé après avoir perdu ses 3 points de vie
4. Le dernier joueur restant est déclaré vainqueur
5. La partie commence automatiquement 10 secondes après qu'au moins 2 joueurs se soient connectés

## Architecture technique
Le jeu utilise une architecture client-serveur avec le protocole UDP pour les communications réseau. Le serveur gère la logique du jeu, la synchronisation des états et la diffusion des mises à jour aux clients. Les clients se connectent au serveur, reçoivent les mises à jour et envoient les actions des joueurs.

### Serveur
- Gère les connexions des clients
- Maintient l'état du jeu et du labyrinthe
- Traite les actions des joueurs
- Diffuse les mises à jour à tous les clients
- Détecte les collisions et calcule les dégâts

### Client
- Se connecte au serveur
- Rend la scène de jeu en 3D
- Capture les entrées de l'utilisateur
- Envoie les actions au serveur
- Affiche l'état du jeu reçu du serveur

## Niveaux de difficulté
1. **Niveau 1** : Labyrinthe simple avec peu de murs et de couloirs larges
2. **Niveau 2** : Labyrinthe de taille moyenne avec plus de murs et quelques impasses
3. **Niveau 3** : Labyrinthe complexe avec de nombreuses impasses et passages étroits

## Performances
Le jeu est optimisé pour maintenir un taux de rafraîchissement supérieur à 50 FPS, même avec un nombre élevé de joueurs. L'affichage du FPS est visible en permanence dans l'interface utilisateur.

## Dépannage

### Problèmes de connexion
- Vérifiez que le serveur est bien lancé avant de démarrer les clients
- Assurez-vous que le port utilisé par le serveur est ouvert dans votre pare-feu
- Si vous jouez sur un réseau local, vérifiez que tous les appareils sont sur le même réseau

### Problèmes de performance
- Réduisez la résolution graphique depuis le menu des options
- Fermez les applications en arrière-plan qui consomment des ressources
- Vérifiez que votre matériel répond aux exigences minimales

## Contribution
Le projet a ete fait avec:
- [@Louis Sebastian Malack](https://github.com/Steb1)
- [@Serigne Saliou Mbacke Mbaye](https://github.com/SSMM0498)
- [@Papa Abdoulaye Diop](https://github.com/papa-abdoulaye-diop)
Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou à soumettre une pull request.

## Licence
Ce projet est sous licence MIT. Voir le fichier LICENSE pour plus de détails.

## Crédits
Inspiré du jeu classique Maze Wars, développé à l'origine au Ames Research Center de la NASA dans les années 1970.
