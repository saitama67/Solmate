from datetime import datetime
import cv2
# import matplotlib.pyplot as plt
from flask import Flask,request
from dotenv import dotenv_values
import requests
import io
from PIL import Image
import calendar
import os
app=Flask(__name__)

# dotenv_path = join(dirname(__file__), '.env')
# load_dotenv(dotenv_path)
months=[
"January"   ,
"February",
"March",
"April",
"May",
"June",
"July",
"August",
"September",
"October",
"November",
"December",
    
]
keys_config=dotenv_values(".env")
values=["name1","name2","witness1","witness2"]
@app.route("/",methods=["POST"])
def generateImage():
    image=cv2.imread("./Card-01.png")
    print(image.shape)
    mapper={}
    fontScale = 3.5
    color = (0, 0, 0)

    data=requests.get("https://api.typeform.com/forms/pxqoXubj/responses",headers={
    "Authorization":f"Bearer {keys_config['token']}"
    })
    data=data.json()

    for index,i in enumerate(data["items"][0]["answers"]):
        mapper[values[index]]=i["text"]

    t=datetime.now()
    mapper["day"]=t.day
    mapper["month"]=months[t.month]
    mapper["year"]=t.year


    loc={
    "name1":(1010,1660),
    "name2":(2650,1660),
    "day":(1300,2050),
    "month":(2100,2050),
    "year":(3500,2050),
    "witness1":(1010,2465),
    "witness2":(2650,2465)
    }
    # filename=f'{mapper["name1"]}-{mapper["name2"]}cert.jpeg'

    thickness = 3
    font=cv2.FONT_HERSHEY_COMPLEX
    for i in mapper.keys():
        cv2.putText(image,str(mapper[i]).title(),loc[i],font, fontScale, 
                    color, thickness, cv2.LINE_AA, False)

    url ="https://api.pinata.cloud/pinning/pinFileToIPFS"
    filename=f"{mapper['name1']}-{mapper['name2']}.jpeg"
    cv2.imwrite(filename,image)
    data=requests.post(url,headers={
        # "Content-Type": f"multipart/form-data; boundary:",
        "pinata_api_key":keys_config["pinata_api_key"],
        "pinata_secret_api_key": keys_config["pinata_secret_api_key"]
    },files={"file":open(filename,"rb")})

    final_data=data.json()
    os.remove(filename)

    # print(final_data)
    return {"IpfsHash":final_data["IpfsHash"]}


# eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiJmMDcyMGY3OS02OWFlLTQwMmQtOTlkNS05M2QwNmM3YjBiNWQiLCJlbWFpbCI6ImRpeGl0YW5pa2V0MTk5QGdtYWlsLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJwaW5fcG9saWN5Ijp7InJlZ2lvbnMiOlt7ImlkIjoiTllDMSIsImRlc2lyZWRSZXBsaWNhdGlvbkNvdW50IjoxfV0sInZlcnNpb24iOjF9LCJtZmFfZW5hYmxlZCI6ZmFsc2V9LCJhdXRoZW50aWNhdGlvblR5cGUiOiJzY29wZWRLZXkiLCJzY29wZWRLZXlLZXkiOiI5OWIyYjFlY2Q5ZjMzMDExNTM0MCIsInNjb3BlZEtleVNlY3JldCI6IjMxMTQyYzc2MTU4MzMxNTA0MTlkMDQzNjA3OGIyYTkzOWZhNDk4NzAxYTA0ODkzYTU2YWRjYmYyZDIwODdhMjciLCJpYXQiOjE2MzA2Njk4MDh9.cXDIiK3w4tgZGphH-zZV088SyRHFI6rTydcHp2VGqA8
# 31142c7615833150419d0436078b2a939fa498701a04893a56adcbf2d2087a27
# # 99b2b1ecd9f330115340

if __name__=="__main__":
	app.run()

