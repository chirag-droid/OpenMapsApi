FROM overv/openstreetmap-tile-server AS runtime

ARG AUTOMATIC_UPDATES=disabled
ARG DOWNLOAD_PBF=https://download.geofabrik.de/asia/india-latest.osm.pbf
ARG DOWNLOAD_POLY=https://download.geofabrik.de/asia/india.poly
ARG FLAT_NODES=enabled

ENV UPDATES=${AUTOMATIC_UPDATES}
ENV DOWNLOAD_PBF=${DOWNLOAD_PBF}
ENV DOWNLOAD_POLY=${DOWNLOAD_POLY}
ENV FLAT_NODES=${FLAT_NODES}

RUN /run.sh import
VOLUME /data/database

ENV POSTGRES_DB=${POSTGRES_DB}
ENV POSTGRES_USER=${POSTGRES_USER}
ENV POSTGRES_PASSWORD=${POSTGRES_PASSWORD}

ENV REPLICATION_URL=https://planet.openstreetmap.org/replication/minute/
ENV MAX_INTERVAL_SECONDS=60
ENV UPDATES=${AUTOMATIC_UPDATES}

ENTRYPOINT [ "/run.sh" ]
CMD ["run"]

EXPOSE 80 5432