import json

# function to add to JSON
def write_json(new_data, filename='data.json'):
    old_releases = []
    new_releases = []
    with open(filename,'r+') as file:
        file_data = json.load(file)
        if artist in file_data:
            if file_data[artist]["mbid"] != None:
                file_data[artist]["mbid"] = (new_data[artist]["mbid"])

            if file_data[artist]["artist_type"] != None:
                file_data[artist]["artist_type"] = (new_data[artist]["artist_type"])

            for release in file_data[artist]["releases"]:
                if release not in new_data[artist]["releases"]:
                    file_data[artist]["releases"].append(release)
        else:
            file_data.append(new_data)

        file.seek(0)

        json.dump(file_data, file, indent=4)

    # python object to be appended
with open("temp.json", 'r+') as file:
    temp_json = json.load(file)
    artist = list(temp_json.keys())[0]
     
write_json(temp_json)

#print(temp_json[artist]["releases"][0]["name"])
