FROM haskell:slim

WORKDIR /app

COPY . /app
RUN cabal update && cabal build && cabal install --lib
