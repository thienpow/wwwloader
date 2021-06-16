#!/bin/sh

image_name="tiny-test"
docker build --rm -t $image_name .
docker run -p 3030:3030 tiny-test