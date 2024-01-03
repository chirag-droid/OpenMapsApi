FROM nginx:alpine AS runtime

ARG BACKEND_PROXY=http://backend:8080
ARG FRONTEND_PROXY=http://frontend:3000
ARG PORT=80

COPY ./nginx.conf.template /etc/nginx/nginx.conf.template
RUN envsubst '$PORT $FRONTEND_PROXY $BACKEND_PROXY' < /etc/nginx/nginx.conf.template > /etc/nginx/nginx.conf

EXPOSE ${PORT}
