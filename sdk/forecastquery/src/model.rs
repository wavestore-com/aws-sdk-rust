// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides information about a forecast. Returned as part of the <code>QueryForecast</code> response.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Forecast  {
    /// <p>The forecast.</p> 
    /// <p>The <i>string</i> of the string-to-array map is one of the following values:</p> 
    /// <ul> 
    /// <li> <p>p10</p> </li> 
    /// <li> <p>p50</p> </li> 
    /// <li> <p>p90</p> </li> 
    /// </ul> 
    /// <p>The default setting is <code>["0.1", "0.5", "0.9"]</code>. Use the optional <code>ForecastTypes</code> parameter of the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateForecast.html">CreateForecast</a> operation to change the values. The values will vary depending on how this is set, with a minimum of <code>1</code> and a maximum of <code>5.</code> </p>
    #[doc(hidden)]
    pub predictions: std::option::Option<std::collections::HashMap<std::string::String, std::vec::Vec<crate::model::DataPoint>>>,
}
impl Forecast {
    /// <p>The forecast.</p> 
    /// <p>The <i>string</i> of the string-to-array map is one of the following values:</p> 
    /// <ul> 
    /// <li> <p>p10</p> </li> 
    /// <li> <p>p50</p> </li> 
    /// <li> <p>p90</p> </li> 
    /// </ul> 
    /// <p>The default setting is <code>["0.1", "0.5", "0.9"]</code>. Use the optional <code>ForecastTypes</code> parameter of the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateForecast.html">CreateForecast</a> operation to change the values. The values will vary depending on how this is set, with a minimum of <code>1</code> and a maximum of <code>5.</code> </p>
    pub fn predictions(&self) -> std::option::Option<& std::collections::HashMap<std::string::String, std::vec::Vec<crate::model::DataPoint>>> {
        self.predictions.as_ref()
    }
}
/// See [`Forecast`](crate::model::Forecast).
pub mod forecast {
    
    /// A builder for [`Forecast`](crate::model::Forecast).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) predictions: std::option::Option<std::collections::HashMap<std::string::String, std::vec::Vec<crate::model::DataPoint>>>,
    }
    impl Builder {
        /// Adds a key-value pair to `predictions`.
        ///
        /// To override the contents of this collection use [`set_predictions`](Self::set_predictions).
        ///
        /// <p>The forecast.</p> 
        /// <p>The <i>string</i> of the string-to-array map is one of the following values:</p> 
        /// <ul> 
        /// <li> <p>p10</p> </li> 
        /// <li> <p>p50</p> </li> 
        /// <li> <p>p90</p> </li> 
        /// </ul> 
        /// <p>The default setting is <code>["0.1", "0.5", "0.9"]</code>. Use the optional <code>ForecastTypes</code> parameter of the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateForecast.html">CreateForecast</a> operation to change the values. The values will vary depending on how this is set, with a minimum of <code>1</code> and a maximum of <code>5.</code> </p>
        pub fn predictions(mut self, k: impl Into<std::string::String>, v: std::vec::Vec<crate::model::DataPoint>) -> Self {
            let mut hash_map = self.predictions.unwrap_or_default();
                            hash_map.insert(k.into(), v);
                            self.predictions = Some(hash_map);
                            self
        }
        /// <p>The forecast.</p> 
        /// <p>The <i>string</i> of the string-to-array map is one of the following values:</p> 
        /// <ul> 
        /// <li> <p>p10</p> </li> 
        /// <li> <p>p50</p> </li> 
        /// <li> <p>p90</p> </li> 
        /// </ul> 
        /// <p>The default setting is <code>["0.1", "0.5", "0.9"]</code>. Use the optional <code>ForecastTypes</code> parameter of the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateForecast.html">CreateForecast</a> operation to change the values. The values will vary depending on how this is set, with a minimum of <code>1</code> and a maximum of <code>5.</code> </p>
        pub fn set_predictions(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::vec::Vec<crate::model::DataPoint>>>) -> Self {
            self.predictions = input; self
        }
        /// Consumes the builder and constructs a [`Forecast`](crate::model::Forecast).
        pub fn build(self) -> crate::model::Forecast {
            crate::model::Forecast {
                predictions: self.predictions
                ,
            }
        }
    }
    
    
}
impl Forecast {
    /// Creates a new builder-style object to manufacture [`Forecast`](crate::model::Forecast).
    pub fn builder() -> crate::model::forecast::Builder {
        crate::model::forecast::Builder::default()
    }
}

/// <p>The forecast value for a specific date. Part of the <code>Forecast</code> object.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DataPoint  {
    /// <p>The timestamp of the specific forecast.</p>
    #[doc(hidden)]
    pub timestamp: std::option::Option<std::string::String>,
    /// <p>The forecast value.</p>
    #[doc(hidden)]
    pub value: std::option::Option<f64>,
}
impl DataPoint {
    /// <p>The timestamp of the specific forecast.</p>
    pub fn timestamp(&self) -> std::option::Option<& str> {
        self.timestamp.as_deref()
    }
    /// <p>The forecast value.</p>
    pub fn value(&self) -> std::option::Option<f64> {
        self.value
    }
}
/// See [`DataPoint`](crate::model::DataPoint).
pub mod data_point {
    
    /// A builder for [`DataPoint`](crate::model::DataPoint).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) timestamp: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<f64>,
    }
    impl Builder {
        /// <p>The timestamp of the specific forecast.</p>
        pub fn timestamp(mut self, input: impl Into<std::string::String>) -> Self {
            self.timestamp = Some(input.into());
            self
        }
        /// <p>The timestamp of the specific forecast.</p>
        pub fn set_timestamp(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.timestamp = input; self
        }
        /// <p>The forecast value.</p>
        pub fn value(mut self, input: f64) -> Self {
            self.value = Some(input);
            self
        }
        /// <p>The forecast value.</p>
        pub fn set_value(mut self, input: std::option::Option<f64>) -> Self {
            self.value = input; self
        }
        /// Consumes the builder and constructs a [`DataPoint`](crate::model::DataPoint).
        pub fn build(self) -> crate::model::DataPoint {
            crate::model::DataPoint {
                timestamp: self.timestamp
                ,
                value: self.value
                ,
            }
        }
    }
    
    
}
impl DataPoint {
    /// Creates a new builder-style object to manufacture [`DataPoint`](crate::model::DataPoint).
    pub fn builder() -> crate::model::data_point::Builder {
        crate::model::data_point::Builder::default()
    }
}

