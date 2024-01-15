FROM debian:bullseye-slim as builder

ARG DOWNLOAD_PBF=https://download.geofabrik.de/asia/india-latest.osm.pbf

ENV DOWNLOAD_PBF=${DOWNLOAD_PBF}

WORKDIR /data

RUN wget ${DOWNLOAD_PBF} -o data.osm.pbf

RUN osrm-extract -p /opt/car.lua /data/data.osm.pbf
