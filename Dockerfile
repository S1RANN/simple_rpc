FROM rust:1-bullseye as builder
WORKDIR /opt/simple_rpc
COPY . .
RUN cd client && cargo build --release && cd ..
RUN cd server && cargo build --release && cd ..
RUN mkdir /opt/simple_rpc/bin 
RUN cp client/target/release/client /opt/simple_rpc/bin/client
RUN cp server/target/release/server /opt/simple_rpc/bin/server

FROM debian:bullseye-slim
COPY --from=builder /opt/simple_rpc/bin/* /usr/local/bin/
COPY --from=builder /opt/simple_rpc/run.sh /usr/local/bin/run.sh
CMD ["run.sh"]