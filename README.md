Cette architecture représente un système de load balancing (répartition de charge) avec plusieurs composants, chacun ayant un rôle spécifique dans la gestion des requêtes client et leur distribution aux serveurs backend. Voici une explication de chaque composant et de la manière dont ils interagissent :

1. Client/User
Description: C'est l'utilisateur ou l'application qui envoie des requêtes au système. Ces requêtes sont généralement des requêtes HTTP, comme les appels à une API ou les demandes de pages web.
Flux: Le client envoie une requête à l'adresse IP et au port du serveur HTTP.
2. HTTP Server (Hyper)
Description: Il s'agit d'un serveur HTTP basé sur la bibliothèque Hyper en Rust. Le serveur reçoit les requêtes HTTP des clients.
Rôle:
Le serveur écoute les requêtes entrantes sur un port spécifique (par exemple, 8080).
Il accepte les connexions des clients et passe les requêtes à un gestionnaire pour traitement.
Flux: Le serveur HTTP reçoit une requête du client et la transmet au Request Handler.
3. Request Handler
Description: Ce composant est responsable de la gestion des requêtes reçues du serveur HTTP.
Rôle:
Il prépare la requête pour l'envoyer à un backend.
Il peut modifier la requête, ajouter des en-têtes, ou effectuer d'autres opérations nécessaires avant d'envoyer la requête au load balancer.
Flux: Le gestionnaire de requêtes envoie la requête préparée au Load Balancer.
4. Load Balancer
Description: Le load balancer (répartiteur de charge) utilise des algorithmes pour distribuer les requêtes entrantes entre plusieurs serveurs backend.
Types de Load Balancing:
RoundRobin Load Balancer: Distribue les requêtes de manière circulaire entre les serveurs backend.
Weighted RoundRobin Load Balancer: Distribue les requêtes en fonction du poids attribué à chaque serveur backend.
Least Connections Load Balancer: Envoie la requête au serveur avec le moins de connexions actives.
Rôle:
Choisit un serveur backend selon l'algorithme configuré.
Transmet la requête au serveur backend choisi.
Flux: Le load balancer distribue la requête au backend approprié.
5. Backend Servers
Description: Ce sont les serveurs qui traitent réellement les requêtes. Ils peuvent héberger des applications, des bases de données ou des services API.
Rôle:
Recevoir les requêtes du load balancer.
Traiter les requêtes et retourner les réponses au load balancer, qui ensuite les retourne au client via le serveur HTTP.
Flux: Le backend retourne une réponse au load balancer après avoir traité la requête.
Résumé de l'Interaction
Client: Envoie une requête HTTP au système.
HTTP Server: Reçoit la requête et la transmet au gestionnaire de requêtes.
Request Handler: Prépare la requête et l'envoie au load balancer.
Load Balancer: Sélectionne un backend selon l'algorithme choisi et transmet la requête à ce backend.
Backend: Traite la requête et renvoie la réponse au load balancer, qui la transmet ensuite au client via le serveur HTTP.
