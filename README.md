# lib-fingerprint

A fingerprint is a unique identifier for a file's contents.

Fingerprints come in multiple "kinds", which are represented by textual identifiers.
Fingerprints themselves are represented as binary blobs.

Fingerprint kinds MUST maintain exact implementation compatibility; once the algorithm for a given kind
has been created and its fingerprints have been crawled, it can't be changed. If a change is needed,
that has to be a new kind of fingerprint.

This rule means that we start out with two kinds that existed prior to this library being created,
which have specific rules about how to compute the fingerprint, and specific text identifiers.

For more information, refer to the documentation for the types below.
