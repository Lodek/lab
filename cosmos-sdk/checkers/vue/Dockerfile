FROM node:14-bullseye-slim

WORKDIR /app

COPY . /app

RUN apt-get update &&\
    apt-get install --yes python build-essential &&\
    npm install &&\
    npm run build

CMD ["npm", "run", "serve"]
