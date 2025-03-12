# Discrete isolation for labelling population centres on maps

The purpose of this repository is to calculate discrete isolation values in order to load labels on a map at different zoom levels.

The algorithm is fairly simple and a good explanation is provided here: https://link.springer.com/article/10.1007/s42489-021-00079-y

Essentially, we loop through each city in descending order of importance (in this repository I've specifically chosen population as a marker for importance) and then identify the closest city which is more important than the current city, and use distance as a metric for isolation.

E.g. for Tokyo, which has the highest population in our data set, the isolation value is infinite, or whatever we cap it at, as there is no city with a greater population in the dataset. For Paris, the discrete isolation value the distance to London, as London is the closest city that has a higher population than Paris.

## Data

The source of data for the cities comes from `./files/worldcities.csv` which is provided by https://simplemaps.com/data/world-cities under Creative Commons Attribution 4.0

## Code

For calculating the isolation values themselves, there is the script I wrote in python available `calculate_isolation_values.py` but it's really slow, because it's in python. Then I rewrote it in rust as main.rs. The algorithm is `O(nlog(n))` complexity.

I've also provided a script that calculates **Mapbox** zoom levels when labels should appear assuming a label width of 75px. `calculate_mapbox_zoom.py`
.
