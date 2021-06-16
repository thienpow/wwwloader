#!/bin/sh


docker login
docker build --rm -t thienpow/wwwloader .
docker push thienpow/wwwloader