#[allow(unused_macros)]
macro_rules! define_codepoint_set {
    (
        $(#[$meta:meta])*
        pub struct $name:ident => $data:path;
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone)]
        pub struct $name {
            codepoints: $crate::CodePoints,
        }

        impl $name {
            /// Creates a new instance of this character set.
            pub fn new() -> Self {
                Self {
                    codepoints: $crate::CodePoints::from_slice($data),
                }
            }

            /// Returns a cached static reference to this character set.
            ///
            /// The instance is initialized on first access via
            /// [`std::sync::OnceLock`]; subsequent calls return the same
            /// reference with no allocation.
            pub fn cached() -> &'static Self {
                static INSTANCE: std::sync::OnceLock<$name> = std::sync::OnceLock::new();
                INSTANCE.get_or_init(Self::new)
            }

            /// Returns `true` if every character in `text` belongs to this set.
            pub fn contains(&self, text: &str) -> bool {
                self.codepoints.contains(text)
            }

            /// Returns the underlying [`CodePoints`](crate::CodePoints) collection.
            pub fn codepoints(&self) -> &$crate::CodePoints {
                &self.codepoints
            }

            /// Returns all code points as a `Vec<u32>`.
            ///
            /// The order of elements is not guaranteed because the underlying
            /// collection is a hash set. Prefer [`Self::codepoints`] for
            /// membership checks.
            pub fn codepoints_vec(&self) -> Vec<u32> {
                self.codepoints.iter().copied().collect()
            }

            /// Validates that every character in `text` belongs to this set.
            ///
            /// Returns `Ok(())` on success, or a [`ValidationError`](crate::ValidationError)
            /// identifying the first character that does not belong.
            pub fn validate(&self, text: &str) -> Result<(), $crate::validation::ValidationError> {
                self.codepoints.validate(text)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
