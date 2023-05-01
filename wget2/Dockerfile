FROM debian:buster-slim as build
RUN apt-get update && apt install -y git libtool autopoint flex lzip texinfo gettext pkg-config wget build-essential libgnutls28-dev
RUN git clone https://gitlab.com/gnuwget/wget2.git
WORKDIR /wget2
RUN ./bootstrap
RUN ./configure --prefix=/usr/local
RUN make
RUN make install

FROM debian:buster-slim
ENV LD_LIBRARY_PATH=/usr/local/lib
RUN apt-get update && apt install -y libssl1.1
COPY --from=build /usr/local/lib/libwget.so.1 /usr/local/lib/
COPY --from=build /usr/lib/x86_64-linux-gnu/*.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/local/bin/wget2 /usr/local/bin/wget2
ENTRYPOINT [ "/usr/local/bin/wget2" ]
