// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>The GetEntitlementsRequest contains results from the GetEntitlements operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetEntitlementsOutput {
    /// <p>The set of entitlements found through the GetEntitlements operation. If the result contains an empty set of entitlements, NextToken might still be present and should be used.</p>
    pub entitlements: std::option::Option<std::vec::Vec<crate::model::Entitlement>>,
    /// <p>For paginated results, use NextToken in subsequent calls to GetEntitlements. If the result contains an empty set of entitlements, NextToken might still be present and should be used.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl GetEntitlementsOutput {
    /// <p>The set of entitlements found through the GetEntitlements operation. If the result contains an empty set of entitlements, NextToken might still be present and should be used.</p>
    pub fn entitlements(&self) -> std::option::Option<&[crate::model::Entitlement]> {
        self.entitlements.as_deref()
    }
    /// <p>For paginated results, use NextToken in subsequent calls to GetEntitlements. If the result contains an empty set of entitlements, NextToken might still be present and should be used.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for GetEntitlementsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetEntitlementsOutput");
        formatter.field("entitlements", &self.entitlements);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`GetEntitlementsOutput`](crate::output::GetEntitlementsOutput)
pub mod get_entitlements_output {
    /// A builder for [`GetEntitlementsOutput`](crate::output::GetEntitlementsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) entitlements: std::option::Option<std::vec::Vec<crate::model::Entitlement>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `entitlements`.
        ///
        /// To override the contents of this collection use [`set_entitlements`](Self::set_entitlements).
        ///
        /// <p>The set of entitlements found through the GetEntitlements operation. If the result contains an empty set of entitlements, NextToken might still be present and should be used.</p>
        pub fn entitlements(mut self, input: crate::model::Entitlement) -> Self {
            let mut v = self.entitlements.unwrap_or_default();
            v.push(input);
            self.entitlements = Some(v);
            self
        }
        /// <p>The set of entitlements found through the GetEntitlements operation. If the result contains an empty set of entitlements, NextToken might still be present and should be used.</p>
        pub fn set_entitlements(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Entitlement>>,
        ) -> Self {
            self.entitlements = input;
            self
        }
        /// <p>For paginated results, use NextToken in subsequent calls to GetEntitlements. If the result contains an empty set of entitlements, NextToken might still be present and should be used.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>For paginated results, use NextToken in subsequent calls to GetEntitlements. If the result contains an empty set of entitlements, NextToken might still be present and should be used.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`GetEntitlementsOutput`](crate::output::GetEntitlementsOutput)
        pub fn build(self) -> crate::output::GetEntitlementsOutput {
            crate::output::GetEntitlementsOutput {
                entitlements: self.entitlements,
                next_token: self.next_token,
            }
        }
    }
}
impl GetEntitlementsOutput {
    /// Creates a new builder-style object to manufacture [`GetEntitlementsOutput`](crate::output::GetEntitlementsOutput)
    pub fn builder() -> crate::output::get_entitlements_output::Builder {
        crate::output::get_entitlements_output::Builder::default()
    }
}
