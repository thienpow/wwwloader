FROM alpine:3.13.5

COPY ./.build/wwwloader /wwwloader
RUN chmod u+x /wwwloader

COPY ./www /www

COPY ./start.sh /start
RUN chmod u+x /start

CMD ["/start"]