FROM rust

#copy the file

COPY ./src/ /app/src/
COPY ./Yolo_front/ /app/front/
COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
COPY ./request.js /app/request.js
COPY ./trash-bin.png /app/trash
COPY ./Yolo_front/pkg/ /app/pkg/

RUN apt-get update

WORKDIR /app/

RUN curl -fsSL https://deb.nodesource.com/setup_14.x | bash -
RUN apt install -y nodejs

WORKDIR /app/front/ 
RUN npm install
EXPOSE 9000
USER 1000
CMD ["node", "server.js"]