import pandas as pd
import numpy as np
from haversine import haversine
import time

# Earth's circumference in km (approximate)
EARTH_CIRCUMFERENCE_KM = 40075.0
HALF_CIRCUMFERENCE_KM = EARTH_CIRCUMFERENCE_KM / 2

df = pd.read_csv("../files/worldcities.csv")

def calculate_distance(lat1, lng1, lat2, lng2):
    return haversine((lat1, lng1), (lat2, lng2))

df = df.sort_values(by="population", ascending=False).reset_index(drop=True)

# Max possible distance (approx) between two points
df["isolation_value"] = HALF_CIRCUMFERENCE_KM

for i, city in df.iterrows():
    
    lat1, lng1, pop1 = city["lat"], city["lng"], city["population"]
    for j, other_city in df.iterrows():
        if other_city["population"] > pop1:  # Only consider cities with higher population
            distance = calculate_distance(lat1, lng1, other_city["lat"], other_city["lng"])
            if distance < df.at[i, "isolation_value"]:
                df.at[i, "isolation_value"] = distance

df.to_csv("city_isolation_values.csv", index=False)

