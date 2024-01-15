FROM ghcr.io/project-osrm/osrm-backend as builder

RUN apt-get update && apt-get install -y wget

ARG DOWNLOAD_PBF=https://download.geofabrik.de/asia/india-latest.osm.pbf

ENV DOWNLOAD_PBF=${DOWNLOAD_PBF}

RUN mkdir -p data
WORKDIR /data

RUN wget ${DOWNLOAD_PBF} -O data.osm.pbf

RUN osrm-extract -p /opt/car.lua /data/data.osm.pbf
RUN osrm-partition /data/data.osrm
RUN osrm-customize /data/data.osrm

CMD ["osrm-routed", "--algorithm", "mld", "/data/data.osrm"]