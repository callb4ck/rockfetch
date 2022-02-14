#!/usr/bin/perl

$container = `podman -v 2> /dev/null` ? "podman" : "docker";

foreach (`podman ps`) {
    next if $_ =~ "CONTAINER ID";
    $_ =~ /\s/;

    system "$container cp ./target/x86_64-unknown-linux-musl/release/rockfetch $`:/";
}
