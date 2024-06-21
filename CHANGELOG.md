# v3.0.0

- Changes the Serde serialization of `Kind` to match Strum's.

# v2.1.0

- Adds new `Kind::JarMavenCentralV1` fingerprint.
- Attempts to improve performance of file-based fingerprinting by paralellizing across threads.
  - This was done now that we have several kinds of fingerprints, and we'll probably just keep adding more.

# v2.0.0

Refactored to the new Sparkle-based view of fingerprints.

# v1.0.0

Initial version, built for VSI specifically.
