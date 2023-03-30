// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct SendAlexaOfferToMasterOutput  {
    /// <p>The base64-encoded SDP answer content.</p>
    #[doc(hidden)]
    pub answer: std::option::Option<std::string::String>,
}
impl SendAlexaOfferToMasterOutput {
    /// <p>The base64-encoded SDP answer content.</p>
    pub fn answer(&self) -> std::option::Option<& str> {
        self.answer.as_deref()
    }
}
/// See [`SendAlexaOfferToMasterOutput`](crate::output::SendAlexaOfferToMasterOutput).
pub mod send_alexa_offer_to_master_output {
    
    /// A builder for [`SendAlexaOfferToMasterOutput`](crate::output::SendAlexaOfferToMasterOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) answer: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The base64-encoded SDP answer content.</p>
        pub fn answer(mut self, input: impl Into<std::string::String>) -> Self {
            self.answer = Some(input.into());
            self
        }
        /// <p>The base64-encoded SDP answer content.</p>
        pub fn set_answer(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.answer = input; self
        }
        /// Consumes the builder and constructs a [`SendAlexaOfferToMasterOutput`](crate::output::SendAlexaOfferToMasterOutput).
        pub fn build(self) -> crate::output::SendAlexaOfferToMasterOutput {
            crate::output::SendAlexaOfferToMasterOutput {
                answer: self.answer
                ,
            }
        }
    }
    
    
}
impl SendAlexaOfferToMasterOutput {
    /// Creates a new builder-style object to manufacture [`SendAlexaOfferToMasterOutput`](crate::output::SendAlexaOfferToMasterOutput).
    pub fn builder() -> crate::output::send_alexa_offer_to_master_output::Builder {
        crate::output::send_alexa_offer_to_master_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetIceServerConfigOutput  {
    /// <p>The list of ICE server information objects.</p>
    #[doc(hidden)]
    pub ice_server_list: std::option::Option<std::vec::Vec<crate::model::IceServer>>,
}
impl GetIceServerConfigOutput {
    /// <p>The list of ICE server information objects.</p>
    pub fn ice_server_list(&self) -> std::option::Option<& [crate::model::IceServer]> {
        self.ice_server_list.as_deref()
    }
}
/// See [`GetIceServerConfigOutput`](crate::output::GetIceServerConfigOutput).
pub mod get_ice_server_config_output {
    
    /// A builder for [`GetIceServerConfigOutput`](crate::output::GetIceServerConfigOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) ice_server_list: std::option::Option<std::vec::Vec<crate::model::IceServer>>,
    }
    impl Builder {
        /// Appends an item to `ice_server_list`.
        ///
        /// To override the contents of this collection use [`set_ice_server_list`](Self::set_ice_server_list).
        ///
        /// <p>The list of ICE server information objects.</p>
        pub fn ice_server_list(mut self, input: crate::model::IceServer) -> Self {
            let mut v = self.ice_server_list.unwrap_or_default();
                            v.push(input);
                            self.ice_server_list = Some(v);
                            self
        }
        /// <p>The list of ICE server information objects.</p>
        pub fn set_ice_server_list(mut self, input: std::option::Option<std::vec::Vec<crate::model::IceServer>>) -> Self {
            self.ice_server_list = input; self
        }
        /// Consumes the builder and constructs a [`GetIceServerConfigOutput`](crate::output::GetIceServerConfigOutput).
        pub fn build(self) -> crate::output::GetIceServerConfigOutput {
            crate::output::GetIceServerConfigOutput {
                ice_server_list: self.ice_server_list
                ,
            }
        }
    }
    
    
}
impl GetIceServerConfigOutput {
    /// Creates a new builder-style object to manufacture [`GetIceServerConfigOutput`](crate::output::GetIceServerConfigOutput).
    pub fn builder() -> crate::output::get_ice_server_config_output::Builder {
        crate::output::get_ice_server_config_output::Builder::default()
    }
}

