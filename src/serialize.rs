//! Contains helpers for serializing.
//!
//! Most serialization work is handled by serde, but we needed additional custom logic for kinds,
//! so here we are.

/// Contains helpers for serializing fingerprint kinds.
pub mod kind {
    use std::collections::HashSet;

    use serde::{Deserialize, Serialize};

    use crate::{CommentStrippedSHA256, RawSHA256};

    /// The stringified version of a [`Kind`].
    #[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
    pub struct SerializedKind(String);

    impl SerializedKind {
        /// Create a new instance from a `String`.
        /// Be careful: this method is intended only for use when serializing;
        /// it is possible to create nonsensical values with this method.
        pub fn new(inner: impl ToString) -> Self {
            Self(inner.to_string())
        }

        /// Extract the inner `String` for this instance.
        pub fn into_inner(self) -> String {
            self.0
        }
    }

    /// Return the kinds used to evaluate a [`crate::Combined`] output by
    /// this version of this crate.
    ///
    /// All kinds _evaluated_ for a `Combined` are included, whether the `Combined`
    /// actually included those kinds or not.
    ///
    /// Specifically: Even if a `Combined` does not concretely include a `CommentStrippedSHA256`
    /// fingerprint, it is still included in the serialized list of kinds, because it was
    /// something that the fingerprint algorithm _considered_ for the file that is
    /// represented by a `Combined` value.
    pub fn kinds_evaluated() -> HashSet<SerializedKind> {
        [RawSHA256.to_string(), CommentStrippedSHA256.to_string()]
            .into_iter()
            .map(SerializedKind)
            .collect()
    }

    /// If the previous set of kinds contains all of the kinds we would now emit
    /// (ignoring kinds we wouldn't emit), we should not re-fingerprint the files.
    pub fn would_evaluate_new_kinds(previously_evaluated: &HashSet<SerializedKind>) -> bool {
        let would_be_evaluated = kinds_evaluated();
        !would_be_evaluated.is_subset(previously_evaluated)
    }
}
