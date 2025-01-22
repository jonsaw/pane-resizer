#!/usr/bin/env bash
leptosfmt --stdin | \
    rustywind --stdin --custom-regex "\b(?:class(?:Name)*\s*=\s*[\"'])([_a-zA-Z0-9\.\s\-:\[\]/]+)[\"']" --output-css-file "$(pwd)/diststyles/styles.css" | \
    rustfmt --edition 2021
