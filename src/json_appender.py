import json

# function to add to JSON
def write_json(new_data, filename='data.json'):
    old_releases = []
    new_releases = []
    with open(filename, 'r+', encoding='utf8') as file:
        file_data = json.load(file)

        if artist in file_data: #checking if that artist is in the json already

            if file_data[artist]["mbid"] != None: #checking if the mbid is already under this artist
                file_data[artist]["mbid"] = (new_data[artist]["mbid"])

            if file_data[artist]["artist_type"] != None: #checking if the artist_type is already under this artist
                file_data[artist]["artist_type"] = (new_data[artist]["artist_type"])

            for release in file_data[artist]["releases"]: #checking to see which releases arent under the artist's releases already
                if release not in new_data[artist]["releases"]:
                    file_data[artist]["releases"].append(release)

        else: #this is a new artist to the json
            file_data[artist] = (new_data)[artist]

        #go back to the beginning of the file
        file.seek(0)

        #write the new json to the file
        json.dump(file_data, file, indent=4)


with open("temp.json", 'r+', encoding='utf8') as file:
    temp_json = json.load(file)
    artist = list(temp_json.keys())[0].title()

write_json(temp_json)