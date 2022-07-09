
## Pourquoi une db sql ?

Ce devlog étant aussi pour moi un moyen de me rappeler de mes choix, et d'exposer mes propres arguments afin de faire ma petite liste de pour et de contre, je pense qu'il est honnête de poser la question suivante : mais pourquoi une db sql plutôt qu'une db clé/valeur ?

Déjà, et au cas où quelqu'un de pas trop tech tomberait sur ce devlog, c'est quoi la différence ?

Commençons par le plus simple, le clé/valeur. Vous voyez ce qu'est un dictionnaire ? Avec un mot = une définition, et c'est rangé par ordre alphabétique ? Et ben c'est ça, sauf que le mot, on appelle ça une clé, et la définition on la remplace par une valeur qui nous intéresse. D'ailleurs le clé/valeur est parfois appelé un `dict`ionnaire dans certains langage. Bien sûr, au niveau technique ça va plus loin, mais l'idée c'est juste ça.

Le sql est un langage de programmation, le `strutured query language`, et fonctionne de pair avec un moteur d'éxecution du sql. Son avantage majeur est la structuration de la données. C'est comme une notice de montage, vous savez de quoi vous avez besoin, ce que vous voulez obtenir, et comment vous allez l'obtenir. Chaque chose est défini.

L'avantage du clé/valeur réside dans sa simplicité de mise en place et d'usage, une fois l'accès à la db ouvert c'est open bar, juste besoin des clés. Le désavantage, c'est que comme la db ne gère rien en terme de structure, tout est à faire coté code. Donc le clé/valeur, c'est plus souple et plus permissif, mais toute la logique propre aux données n'est pas avec les données.

Maintenant qu'on a vu tout ça, pourquoi je préfère utiliser du sql ?

Parce que la logique sur les données dans l'app est suffisamment importante pour que je préfère la déplacer côté sql plutôt que côté code.

Pour mieux comprendre, reprenons l'exemple de la notice de montage. Si je devais utiliser un dictionnaire (l'autre nom du clé/valeur), je me heurterais à deux éccueils :

- Si je mets beaucoup d'info dans mes valeurs, j'ai moins de clés, mais j'ai plus de chose à faire côté code pour récupérer les données qui m'intéresse, voire je vais devoir récupérer des données en trop, parce que je vais tout récupérer dans la valeur, pas juste la partie utile. C'est comme si ma notice était très courte, avec très peu d'étape, et à chaque étape beaucoup de choses à faire, ce qui entraîne des oublis et des erreurs.
- A l'inverse, si je ne mets pas beaucoup de clés en répartissant les valeurs dans pleins de clés, je vais passer mon temps à passer d'une clé à l'autre pour récupérer toutes les valeurs suffisantes. Tant qu'il n'y en a pas beaucoup, ça va, mais dès que ça augmente, c'est plus complexe. Comme une notice sur 40 pages, et on ne se souvient pas de tête à quel endroit on est, et qu'il faut un marque-page si on veut réussir à s'en sortir.

Et une solution intermédiaire ?

C'est beau de croire au miracle.

Alors oui, en théorie, on peut tenter d'obtenir

??

