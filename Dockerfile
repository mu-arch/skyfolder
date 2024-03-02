FROM clux/muslrust:stable as builder
ADD . /skyfolder
WORKDIR /skyfolder
RUN echo "nobody:x:65534:65534:Nobody:/:" > /etc_passwd && \
    cargo build --release

###################################

FROM scratch
ARG arch=x86_64
COPY --from=builder "/skyfolder/target/${arch}-unknown-linux-musl/release/skyfolder" "./skyfolder"
COPY --from=builder "/etc_passwd" "/etc/passwd"
USER nobody
EXPOSE 30080
ENTRYPOINT [ "./skyfolder" ]