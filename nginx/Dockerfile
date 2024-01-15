FROM nginx:alpine AS runtime

ARG BACKEND_PROXY=http://backend:8080
ARG FRONTEND_PROXY=http://frontend:3000
ARG RENDERER_PROXY=http://renderer:80
ARG ROUTER_PROXY=http://router:5000
ARG PORT=80

COPY ./nginx.conf.template /etc/nginx/nginx.conf.template
RUN envsubst '$PORT $FRONTEND_PROXY $BACKEND_PROXY $RENDERER_PROXY $ROUTER_PROXY' < /etc/nginx/nginx.conf.template > /etc/nginx/nginx.conf

EXPOSE ${PORT}
