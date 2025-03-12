import math
import pandas as pd
import numpy as np

city_isolation_values = pd.read_csv('../files/city_isolation_values.csv')

def calculate_km_per_pixel(latitude, zoom):
    # Earth's radius
    R = 6378.137
    
    # Mapbox starts with 512 px for the full width of the Earth at zoom 0
    tile_size = 512
    
    # Cosine factor for Mercator projection
    cos_lat = math.cos(math.radians(latitude))
    
    # Circumference of Earth at the given latitude, then divide by the total pixels
    # (tile_size * 2^zoom) is the number of pixels across the map at this zoom level.
    return (2 * math.pi * R * cos_lat) / (tile_size * (2 ** zoom))


def calculate_zoom_for_label(label_pixel_width, real_world_distance, latitude=0):
    for zoom in np.arange(0, 10.5, 0.5):  # Mapbox zoom levels in increments of 0.5
        km_per_pixel = calculate_km_per_pixel(latitude,zoom )
        label_distance = label_pixel_width * km_per_pixel
        
        if label_distance <= real_world_distance:
            return zoom
    
    return 21 # Max zoom level as default

# Assumed radius around a label to prevent label collision
label_width_pixels = 75

city_isolation_values["zoom_level"] = city_isolation_values.apply(
    lambda row: calculate_zoom_for_label(label_pixel_width=label_width_pixels, 
                                         real_world_distance=row['isolation_value'], 
                                         latitude=row['lat']), 
    axis=1
)

city_isolation_values.to_csv('../files/city_zoom.csv', index=False)
