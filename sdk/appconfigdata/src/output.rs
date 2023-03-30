// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct StartConfigurationSessionOutput  {
    /// <p>Token encapsulating state about the configuration session. Provide this token to the <code>GetLatestConfiguration</code> API to retrieve configuration data.</p> <important> 
    /// <p>This token should only be used once in your first call to <code>GetLatestConfiguration</code>. You MUST use the new token in the <code>GetLatestConfiguration</code> response (<code>NextPollConfigurationToken</code>) in each subsequent call to <code>GetLatestConfiguration</code>.</p> 
    /// </important>
    #[doc(hidden)]
    pub initial_configuration_token: std::option::Option<std::string::String>,
}
impl StartConfigurationSessionOutput {
    /// <p>Token encapsulating state about the configuration session. Provide this token to the <code>GetLatestConfiguration</code> API to retrieve configuration data.</p> <important> 
    /// <p>This token should only be used once in your first call to <code>GetLatestConfiguration</code>. You MUST use the new token in the <code>GetLatestConfiguration</code> response (<code>NextPollConfigurationToken</code>) in each subsequent call to <code>GetLatestConfiguration</code>.</p> 
    /// </important>
    pub fn initial_configuration_token(&self) -> std::option::Option<& str> {
        self.initial_configuration_token.as_deref()
    }
}
/// See [`StartConfigurationSessionOutput`](crate::output::StartConfigurationSessionOutput).
pub mod start_configuration_session_output {
    
    /// A builder for [`StartConfigurationSessionOutput`](crate::output::StartConfigurationSessionOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) initial_configuration_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Token encapsulating state about the configuration session. Provide this token to the <code>GetLatestConfiguration</code> API to retrieve configuration data.</p> <important> 
        /// <p>This token should only be used once in your first call to <code>GetLatestConfiguration</code>. You MUST use the new token in the <code>GetLatestConfiguration</code> response (<code>NextPollConfigurationToken</code>) in each subsequent call to <code>GetLatestConfiguration</code>.</p> 
        /// </important>
        pub fn initial_configuration_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.initial_configuration_token = Some(input.into());
            self
        }
        /// <p>Token encapsulating state about the configuration session. Provide this token to the <code>GetLatestConfiguration</code> API to retrieve configuration data.</p> <important> 
        /// <p>This token should only be used once in your first call to <code>GetLatestConfiguration</code>. You MUST use the new token in the <code>GetLatestConfiguration</code> response (<code>NextPollConfigurationToken</code>) in each subsequent call to <code>GetLatestConfiguration</code>.</p> 
        /// </important>
        pub fn set_initial_configuration_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.initial_configuration_token = input; self
        }
        /// Consumes the builder and constructs a [`StartConfigurationSessionOutput`](crate::output::StartConfigurationSessionOutput).
        pub fn build(self) -> crate::output::StartConfigurationSessionOutput {
            crate::output::StartConfigurationSessionOutput {
                initial_configuration_token: self.initial_configuration_token
                ,
            }
        }
    }
    
    
}
impl StartConfigurationSessionOutput {
    /// Creates a new builder-style object to manufacture [`StartConfigurationSessionOutput`](crate::output::StartConfigurationSessionOutput).
    pub fn builder() -> crate::output::start_configuration_session_output::Builder {
        crate::output::start_configuration_session_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetLatestConfigurationOutput  {
    /// <p>The latest token describing the current state of the configuration session. This MUST be provided to the next call to <code>GetLatestConfiguration.</code> </p>
    #[doc(hidden)]
    pub next_poll_configuration_token: std::option::Option<std::string::String>,
    /// <p>The amount of time the client should wait before polling for configuration updates again. Use <code>RequiredMinimumPollIntervalInSeconds</code> to set the desired poll interval.</p>
    #[doc(hidden)]
    pub next_poll_interval_in_seconds: i32,
    /// <p>A standard MIME type describing the format of the configuration content.</p>
    #[doc(hidden)]
    pub content_type: std::option::Option<std::string::String>,
    /// <p>The data of the configuration. This may be empty if the client already has the latest version of configuration.</p>
    #[doc(hidden)]
    pub configuration: std::option::Option<aws_smithy_types::Blob>,
}
impl GetLatestConfigurationOutput {
    /// <p>The latest token describing the current state of the configuration session. This MUST be provided to the next call to <code>GetLatestConfiguration.</code> </p>
    pub fn next_poll_configuration_token(&self) -> std::option::Option<& str> {
        self.next_poll_configuration_token.as_deref()
    }
    /// <p>The amount of time the client should wait before polling for configuration updates again. Use <code>RequiredMinimumPollIntervalInSeconds</code> to set the desired poll interval.</p>
    pub fn next_poll_interval_in_seconds(&self) -> i32 {
        self.next_poll_interval_in_seconds
    }
    /// <p>A standard MIME type describing the format of the configuration content.</p>
    pub fn content_type(&self) -> std::option::Option<& str> {
        self.content_type.as_deref()
    }
    /// <p>The data of the configuration. This may be empty if the client already has the latest version of configuration.</p>
    pub fn configuration(&self) -> std::option::Option<& aws_smithy_types::Blob> {
        self.configuration.as_ref()
    }
}
impl  std::fmt::Debug for GetLatestConfigurationOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetLatestConfigurationOutput");
        formatter.field("next_poll_configuration_token", &self.next_poll_configuration_token);
        formatter.field("next_poll_interval_in_seconds", &self.next_poll_interval_in_seconds);
        formatter.field("content_type", &self.content_type);
        formatter.field("configuration", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
/// See [`GetLatestConfigurationOutput`](crate::output::GetLatestConfigurationOutput).
pub mod get_latest_configuration_output {
    
    /// A builder for [`GetLatestConfigurationOutput`](crate::output::GetLatestConfigurationOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
    pub struct Builder {
        pub(crate) next_poll_configuration_token: std::option::Option<std::string::String>,
        pub(crate) next_poll_interval_in_seconds: std::option::Option<i32>,
        pub(crate) content_type: std::option::Option<std::string::String>,
        pub(crate) configuration: std::option::Option<aws_smithy_types::Blob>,
    }
    impl Builder {
        /// <p>The latest token describing the current state of the configuration session. This MUST be provided to the next call to <code>GetLatestConfiguration.</code> </p>
        pub fn next_poll_configuration_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_poll_configuration_token = Some(input.into());
            self
        }
        /// <p>The latest token describing the current state of the configuration session. This MUST be provided to the next call to <code>GetLatestConfiguration.</code> </p>
        pub fn set_next_poll_configuration_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_poll_configuration_token = input; self
        }
        /// <p>The amount of time the client should wait before polling for configuration updates again. Use <code>RequiredMinimumPollIntervalInSeconds</code> to set the desired poll interval.</p>
        pub fn next_poll_interval_in_seconds(mut self, input: i32) -> Self {
            self.next_poll_interval_in_seconds = Some(input);
            self
        }
        /// <p>The amount of time the client should wait before polling for configuration updates again. Use <code>RequiredMinimumPollIntervalInSeconds</code> to set the desired poll interval.</p>
        pub fn set_next_poll_interval_in_seconds(mut self, input: std::option::Option<i32>) -> Self {
            self.next_poll_interval_in_seconds = input; self
        }
        /// <p>A standard MIME type describing the format of the configuration content.</p>
        pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.content_type = Some(input.into());
            self
        }
        /// <p>A standard MIME type describing the format of the configuration content.</p>
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.content_type = input; self
        }
        /// <p>The data of the configuration. This may be empty if the client already has the latest version of configuration.</p>
        pub fn configuration(mut self, input: aws_smithy_types::Blob) -> Self {
            self.configuration = Some(input);
            self
        }
        /// <p>The data of the configuration. This may be empty if the client already has the latest version of configuration.</p>
        pub fn set_configuration(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
            self.configuration = input; self
        }
        /// Consumes the builder and constructs a [`GetLatestConfigurationOutput`](crate::output::GetLatestConfigurationOutput).
        pub fn build(self) -> crate::output::GetLatestConfigurationOutput {
            crate::output::GetLatestConfigurationOutput {
                next_poll_configuration_token: self.next_poll_configuration_token
                ,
                next_poll_interval_in_seconds: self.next_poll_interval_in_seconds
                    .unwrap_or_default()
                ,
                content_type: self.content_type
                ,
                configuration: self.configuration
                ,
            }
        }
    }
    impl std::fmt::Debug for Builder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut formatter = f.debug_struct("Builder");
            formatter.field("next_poll_configuration_token", &self.next_poll_configuration_token);
            formatter.field("next_poll_interval_in_seconds", &self.next_poll_interval_in_seconds);
            formatter.field("content_type", &self.content_type);
            formatter.field("configuration", &"*** Sensitive Data Redacted ***");
            formatter.finish()
        }
    }
    
    
}
impl GetLatestConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`GetLatestConfigurationOutput`](crate::output::GetLatestConfigurationOutput).
    pub fn builder() -> crate::output::get_latest_configuration_output::Builder {
        crate::output::get_latest_configuration_output::Builder::default()
    }
}

