with (import <nixpkgs> {});
mkShell {
  nativeBuildInputs = [
        git
        vim
        python311
        poetry
        arrow-glib
        sbt
        mill
        gradle
        rustc
        cargo
        sops
        apacheKafka_3_2
        (postgresql_15.withPackages ( p: [ p.postgis p.timescaledb ]))
        libiconv
    ];

        # Post Shell Hook
    shellHook = ''
        echo "Using ${postgresql_15.name}."
        echo 'Beginning Startup'
        echo 'Starting ZOOKEEPER'
        zookeeper-server-start infra/kafka/zookeeper.properties > /dev/null &
        echo 'STARTING KAFKA'
        kafka-server-start infra/kafka/server.properties > /dev/null &
    '';
}
