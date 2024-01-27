import json
from pymongo import MongoClient
import certifi
from pymongo.mongo_client import MongoClient
from pymongo.server_api import ServerApi

uri = "mongodb+srv://Julien:rxBv0fhgEyrdnXYt@clusterlocaleat.ywvwbpn.mongodb.net/?retryWrites=true&w=majority"

# Create a new client and connect to the server
client = MongoClient(uri, tlsCAFile=certifi.where())
db = client['rustDB']
collection = db['Point']

# Send a ping to confirm a successful connection
try:
    client.admin.command('ping')
    print("Pinged your deployment. You successfully connected to MongoDB!")
except Exception as e:
    print(e)

 # Charger le fichier JSON
with open('data_filtre.json', 'r') as fichier_json:
    donnees = json.load(fichier_json)

# Insérer chaque élément dans la collection MongoDB
for element in donnees:
    
    el = {
            'title' : element['name'],
            'pointtype' : element['type'],
            'position':
                {
                    'Location_coordinates' : 
                    [
                        element['X'], 
                        element['Y']
                    ],
                    'Location_type' : 'Point'
                }
          }

    if collection.find_one({"title"  : element['name']}) == None:
        collection.insert_one(el)


print('fini')