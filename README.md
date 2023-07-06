# Tor Operator

Tor Operator is a Kubernetes Operator that manages [Onion Balances](https://agabani.github.io/tor-operator/docs/custom_resource_definitions/onionbalance/), [Onion Keys](https://agabani.github.io/tor-operator/docs/custom_resource_definitions/onionkey/) and [Onion Services](https://agabani.github.io/tor-operator/docs/custom_resource_definitions/onionservice/) to provide a highly available, load balanced and fault tolerate [Tor Ingress](https://agabani.github.io/tor-operator/docs/custom_resource_definitions/toringress/).

## Documentation

[https://agabani.github.io/tor-operator/docs/](https://agabani.github.io/tor-operator/docs/)

<!--getting-started-start-->

## Installation

1.  Add the chart repository.

        helm repo add agabani-tor-operator https://agabani.github.io/tor-operator

1.  Install the Tor Operator.

        helm install tor-operator agabani-tor-operator/tor-operator \
            --create-namespace \
            --namespace tor-operator

## Creating a Tor Ingress

1.  Prepare your existing Onion Key to look like:

    - `hostname`
    - `hs_ed25519_public_key`
    - `hs_ed25519_public_key`

    or generate a new Onion Key using:

        cargo install --git https://github.com/agabani/tor-operator --tag 0.0.2
        tor-operator onion-key generate

1.  Create a `Secret` containing the Onion Key.

        kubectl create secret generic tor-ingress-example \
            --from-file=hostname=./hostname \
            --from-file=hs_ed25519_public_key=./hs_ed25519_public_key \
            --from-file=hs_ed25519_secret_key=./hs_ed25519_secret_key

1.  Create an `OnionKey` wrapping the `Secret`.

        # onionkey.yaml
        apiVersion: tor.agabani.co.uk/v1
        kind: OnionKey
        metadata:
          name: tor-ingress-example
        spec:
          secret:
            name: tor-ingress-example

    `kubectl apply -f onionkey.yaml`

1.  Create a `TorIngress`, changing `example:80` to your targets `host:port`

        # toringress.yaml
        apiVersion: tor.agabani.co.uk/v1
        kind: TorIngress
        metadata:
          name: tor-ingress-example
        spec:
          onionBalance:
            onionKey:
              name: tor-ingress-example
          onionService:
            ports:
              - target: example:80
                virtport: 80
            replicas: 3

    `kubectl apply -f toringress.yaml`

<!--getting-started-end-->
