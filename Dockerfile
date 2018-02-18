FROM rustlang/rust:nightly

EXPOSE 80
WORKDIR /usr/src/arschive
COPY . .
RUN cargo install

CMD ROCKET_ENV=production arschive
