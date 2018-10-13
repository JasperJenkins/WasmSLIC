# SLIC Superpixels

Simple Linear Iterative Clustering algorithm made with Rust compiled to WASM.
Bundled [wasm-pack](https://github.com/rustwasm/wasm-pack) with [parcel](https://parceljs.org/)

TL;DR: k-means over a 5-dimensional space, two xy image coordinates and three cielab color coordinates. 
The k-means only has to consider n near pixels for each centroid because far centroids could not be closer no matter the color similarity.
The clusters may not all be connected, so follow with a depth first search and connect smaller disjoint segments to adjacent ones.

[Radhakrishna Achanta, Appu Shaji, Kevin Smith, Aurelien
Lucchi, Pascal Fua, and Sabine S¨usstrunk, SLIC Superpixels, EPFL Technical
Report 149300, June 2010.](http://www.kev-smith.com/papers/SLIC_Superpixels.pdf)
