// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Represents user metadata added to a Users dataset using the <code>PutUsers</code> API. For more information see <a href="https://docs.aws.amazon.com/personalize/latest/dg/importing-users.html">Importing Users Incrementally</a>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct User {
    /// <p>The ID associated with the user.</p>
    pub user_id: std::option::Option<std::string::String>,
    /// <p>A string map of user-specific metadata. Each element in the map consists of a key-value pair. For example, <code>{"numberOfVideosWatched": "45"}</code>.</p>
    /// <p>The keys use camel case names that match the fields in the schema for the Users dataset. In the previous example, the <code>numberOfVideosWatched</code> matches the 'NUMBER_OF_VIDEOS_WATCHED' field defined in the Users schema. For categorical string data, to include multiple categories for a single user, separate each category with a pipe separator (<code>|</code>). For example, <code>\"Member|Frequent shopper\"</code>.</p>
    pub properties: std::option::Option<std::string::String>,
}
impl User {
    /// <p>The ID associated with the user.</p>
    pub fn user_id(&self) -> std::option::Option<&str> {
        self.user_id.as_deref()
    }
    /// <p>A string map of user-specific metadata. Each element in the map consists of a key-value pair. For example, <code>{"numberOfVideosWatched": "45"}</code>.</p>
    /// <p>The keys use camel case names that match the fields in the schema for the Users dataset. In the previous example, the <code>numberOfVideosWatched</code> matches the 'NUMBER_OF_VIDEOS_WATCHED' field defined in the Users schema. For categorical string data, to include multiple categories for a single user, separate each category with a pipe separator (<code>|</code>). For example, <code>\"Member|Frequent shopper\"</code>.</p>
    pub fn properties(&self) -> std::option::Option<&str> {
        self.properties.as_deref()
    }
}
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("User");
        formatter.field("user_id", &self.user_id);
        formatter.field("properties", &self.properties);
        formatter.finish()
    }
}
/// See [`User`](crate::model::User)
pub mod user {
    /// A builder for [`User`](crate::model::User)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) user_id: std::option::Option<std::string::String>,
        pub(crate) properties: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID associated with the user.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.user_id = Some(input.into());
            self
        }
        /// <p>The ID associated with the user.</p>
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.user_id = input;
            self
        }
        /// <p>A string map of user-specific metadata. Each element in the map consists of a key-value pair. For example, <code>{"numberOfVideosWatched": "45"}</code>.</p>
        /// <p>The keys use camel case names that match the fields in the schema for the Users dataset. In the previous example, the <code>numberOfVideosWatched</code> matches the 'NUMBER_OF_VIDEOS_WATCHED' field defined in the Users schema. For categorical string data, to include multiple categories for a single user, separate each category with a pipe separator (<code>|</code>). For example, <code>\"Member|Frequent shopper\"</code>.</p>
        pub fn properties(mut self, input: impl Into<std::string::String>) -> Self {
            self.properties = Some(input.into());
            self
        }
        /// <p>A string map of user-specific metadata. Each element in the map consists of a key-value pair. For example, <code>{"numberOfVideosWatched": "45"}</code>.</p>
        /// <p>The keys use camel case names that match the fields in the schema for the Users dataset. In the previous example, the <code>numberOfVideosWatched</code> matches the 'NUMBER_OF_VIDEOS_WATCHED' field defined in the Users schema. For categorical string data, to include multiple categories for a single user, separate each category with a pipe separator (<code>|</code>). For example, <code>\"Member|Frequent shopper\"</code>.</p>
        pub fn set_properties(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.properties = input;
            self
        }
        /// Consumes the builder and constructs a [`User`](crate::model::User)
        pub fn build(self) -> crate::model::User {
            crate::model::User {
                user_id: self.user_id,
                properties: self.properties,
            }
        }
    }
}
impl User {
    /// Creates a new builder-style object to manufacture [`User`](crate::model::User)
    pub fn builder() -> crate::model::user::Builder {
        crate::model::user::Builder::default()
    }
}

/// <p>Represents item metadata added to an Items dataset using the <code>PutItems</code> API. For more information see <a href="https://docs.aws.amazon.com/personalize/latest/dg/importing-items.html">Importing Items Incrementally</a>. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Item {
    /// <p>The ID associated with the item.</p>
    pub item_id: std::option::Option<std::string::String>,
    /// <p>A string map of item-specific metadata. Each element in the map consists of a key-value pair. For example, <code>{"numberOfRatings": "12"}</code>.</p>
    /// <p>The keys use camel case names that match the fields in the schema for the Items dataset. In the previous example, the <code>numberOfRatings</code> matches the 'NUMBER_OF_RATINGS' field defined in the Items schema. For categorical string data, to include multiple categories for a single item, separate each category with a pipe separator (<code>|</code>). For example, <code>\"Horror|Action\"</code>.</p>
    pub properties: std::option::Option<std::string::String>,
}
impl Item {
    /// <p>The ID associated with the item.</p>
    pub fn item_id(&self) -> std::option::Option<&str> {
        self.item_id.as_deref()
    }
    /// <p>A string map of item-specific metadata. Each element in the map consists of a key-value pair. For example, <code>{"numberOfRatings": "12"}</code>.</p>
    /// <p>The keys use camel case names that match the fields in the schema for the Items dataset. In the previous example, the <code>numberOfRatings</code> matches the 'NUMBER_OF_RATINGS' field defined in the Items schema. For categorical string data, to include multiple categories for a single item, separate each category with a pipe separator (<code>|</code>). For example, <code>\"Horror|Action\"</code>.</p>
    pub fn properties(&self) -> std::option::Option<&str> {
        self.properties.as_deref()
    }
}
impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Item");
        formatter.field("item_id", &self.item_id);
        formatter.field("properties", &self.properties);
        formatter.finish()
    }
}
/// See [`Item`](crate::model::Item)
pub mod item {
    /// A builder for [`Item`](crate::model::Item)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) item_id: std::option::Option<std::string::String>,
        pub(crate) properties: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID associated with the item.</p>
        pub fn item_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.item_id = Some(input.into());
            self
        }
        /// <p>The ID associated with the item.</p>
        pub fn set_item_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.item_id = input;
            self
        }
        /// <p>A string map of item-specific metadata. Each element in the map consists of a key-value pair. For example, <code>{"numberOfRatings": "12"}</code>.</p>
        /// <p>The keys use camel case names that match the fields in the schema for the Items dataset. In the previous example, the <code>numberOfRatings</code> matches the 'NUMBER_OF_RATINGS' field defined in the Items schema. For categorical string data, to include multiple categories for a single item, separate each category with a pipe separator (<code>|</code>). For example, <code>\"Horror|Action\"</code>.</p>
        pub fn properties(mut self, input: impl Into<std::string::String>) -> Self {
            self.properties = Some(input.into());
            self
        }
        /// <p>A string map of item-specific metadata. Each element in the map consists of a key-value pair. For example, <code>{"numberOfRatings": "12"}</code>.</p>
        /// <p>The keys use camel case names that match the fields in the schema for the Items dataset. In the previous example, the <code>numberOfRatings</code> matches the 'NUMBER_OF_RATINGS' field defined in the Items schema. For categorical string data, to include multiple categories for a single item, separate each category with a pipe separator (<code>|</code>). For example, <code>\"Horror|Action\"</code>.</p>
        pub fn set_properties(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.properties = input;
            self
        }
        /// Consumes the builder and constructs a [`Item`](crate::model::Item)
        pub fn build(self) -> crate::model::Item {
            crate::model::Item {
                item_id: self.item_id,
                properties: self.properties,
            }
        }
    }
}
impl Item {
    /// Creates a new builder-style object to manufacture [`Item`](crate::model::Item)
    pub fn builder() -> crate::model::item::Builder {
        crate::model::item::Builder::default()
    }
}

/// <p>Represents user interaction event information sent using the <code>PutEvents</code> API.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Event {
    /// <p>An ID associated with the event. If an event ID is not provided, Amazon Personalize generates a unique ID for the event. An event ID is not used as an input to the model. Amazon Personalize uses the event ID to distinquish unique events. Any subsequent events after the first with the same event ID are not used in model training.</p>
    pub event_id: std::option::Option<std::string::String>,
    /// <p>The type of event, such as click or download. This property corresponds to the <code>EVENT_TYPE</code> field of your Interactions schema and depends on the types of events you are tracking.</p>
    pub event_type: std::option::Option<std::string::String>,
    /// <p>The event value that corresponds to the <code>EVENT_VALUE</code> field of the Interactions schema.</p>
    pub event_value: std::option::Option<f32>,
    /// <p>The item ID key that corresponds to the <code>ITEM_ID</code> field of the Interactions schema.</p>
    pub item_id: std::option::Option<std::string::String>,
    /// <p>A string map of event-specific data that you might choose to record. For example, if a user rates a movie on your site, other than movie ID (<code>itemId</code>) and rating (<code>eventValue</code>) , you might also send the number of movie ratings made by the user.</p>
    /// <p>Each item in the map consists of a key-value pair. For example,</p>
    /// <p> <code>{"numberOfRatings": "12"}</code> </p>
    /// <p>The keys use camel case names that match the fields in the Interactions schema. In the above example, the <code>numberOfRatings</code> would match the 'NUMBER_OF_RATINGS' field defined in the Interactions schema.</p>
    pub properties: std::option::Option<std::string::String>,
    /// <p>The timestamp (in Unix time) on the client side when the event occurred.</p>
    pub sent_at: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The ID of the recommendation.</p>
    pub recommendation_id: std::option::Option<std::string::String>,
    /// <p>A list of item IDs that represents the sequence of items you have shown the user. For example, <code>["itemId1", "itemId2", "itemId3"]</code>.</p>
    pub impression: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl Event {
    /// <p>An ID associated with the event. If an event ID is not provided, Amazon Personalize generates a unique ID for the event. An event ID is not used as an input to the model. Amazon Personalize uses the event ID to distinquish unique events. Any subsequent events after the first with the same event ID are not used in model training.</p>
    pub fn event_id(&self) -> std::option::Option<&str> {
        self.event_id.as_deref()
    }
    /// <p>The type of event, such as click or download. This property corresponds to the <code>EVENT_TYPE</code> field of your Interactions schema and depends on the types of events you are tracking.</p>
    pub fn event_type(&self) -> std::option::Option<&str> {
        self.event_type.as_deref()
    }
    /// <p>The event value that corresponds to the <code>EVENT_VALUE</code> field of the Interactions schema.</p>
    pub fn event_value(&self) -> std::option::Option<f32> {
        self.event_value
    }
    /// <p>The item ID key that corresponds to the <code>ITEM_ID</code> field of the Interactions schema.</p>
    pub fn item_id(&self) -> std::option::Option<&str> {
        self.item_id.as_deref()
    }
    /// <p>A string map of event-specific data that you might choose to record. For example, if a user rates a movie on your site, other than movie ID (<code>itemId</code>) and rating (<code>eventValue</code>) , you might also send the number of movie ratings made by the user.</p>
    /// <p>Each item in the map consists of a key-value pair. For example,</p>
    /// <p> <code>{"numberOfRatings": "12"}</code> </p>
    /// <p>The keys use camel case names that match the fields in the Interactions schema. In the above example, the <code>numberOfRatings</code> would match the 'NUMBER_OF_RATINGS' field defined in the Interactions schema.</p>
    pub fn properties(&self) -> std::option::Option<&str> {
        self.properties.as_deref()
    }
    /// <p>The timestamp (in Unix time) on the client side when the event occurred.</p>
    pub fn sent_at(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.sent_at.as_ref()
    }
    /// <p>The ID of the recommendation.</p>
    pub fn recommendation_id(&self) -> std::option::Option<&str> {
        self.recommendation_id.as_deref()
    }
    /// <p>A list of item IDs that represents the sequence of items you have shown the user. For example, <code>["itemId1", "itemId2", "itemId3"]</code>.</p>
    pub fn impression(&self) -> std::option::Option<&[std::string::String]> {
        self.impression.as_deref()
    }
}
impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Event");
        formatter.field("event_id", &self.event_id);
        formatter.field("event_type", &self.event_type);
        formatter.field("event_value", &self.event_value);
        formatter.field("item_id", &self.item_id);
        formatter.field("properties", &self.properties);
        formatter.field("sent_at", &self.sent_at);
        formatter.field("recommendation_id", &self.recommendation_id);
        formatter.field("impression", &self.impression);
        formatter.finish()
    }
}
/// See [`Event`](crate::model::Event)
pub mod event {
    /// A builder for [`Event`](crate::model::Event)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) event_id: std::option::Option<std::string::String>,
        pub(crate) event_type: std::option::Option<std::string::String>,
        pub(crate) event_value: std::option::Option<f32>,
        pub(crate) item_id: std::option::Option<std::string::String>,
        pub(crate) properties: std::option::Option<std::string::String>,
        pub(crate) sent_at: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) recommendation_id: std::option::Option<std::string::String>,
        pub(crate) impression: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// <p>An ID associated with the event. If an event ID is not provided, Amazon Personalize generates a unique ID for the event. An event ID is not used as an input to the model. Amazon Personalize uses the event ID to distinquish unique events. Any subsequent events after the first with the same event ID are not used in model training.</p>
        pub fn event_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.event_id = Some(input.into());
            self
        }
        /// <p>An ID associated with the event. If an event ID is not provided, Amazon Personalize generates a unique ID for the event. An event ID is not used as an input to the model. Amazon Personalize uses the event ID to distinquish unique events. Any subsequent events after the first with the same event ID are not used in model training.</p>
        pub fn set_event_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.event_id = input;
            self
        }
        /// <p>The type of event, such as click or download. This property corresponds to the <code>EVENT_TYPE</code> field of your Interactions schema and depends on the types of events you are tracking.</p>
        pub fn event_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.event_type = Some(input.into());
            self
        }
        /// <p>The type of event, such as click or download. This property corresponds to the <code>EVENT_TYPE</code> field of your Interactions schema and depends on the types of events you are tracking.</p>
        pub fn set_event_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.event_type = input;
            self
        }
        /// <p>The event value that corresponds to the <code>EVENT_VALUE</code> field of the Interactions schema.</p>
        pub fn event_value(mut self, input: f32) -> Self {
            self.event_value = Some(input);
            self
        }
        /// <p>The event value that corresponds to the <code>EVENT_VALUE</code> field of the Interactions schema.</p>
        pub fn set_event_value(mut self, input: std::option::Option<f32>) -> Self {
            self.event_value = input;
            self
        }
        /// <p>The item ID key that corresponds to the <code>ITEM_ID</code> field of the Interactions schema.</p>
        pub fn item_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.item_id = Some(input.into());
            self
        }
        /// <p>The item ID key that corresponds to the <code>ITEM_ID</code> field of the Interactions schema.</p>
        pub fn set_item_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.item_id = input;
            self
        }
        /// <p>A string map of event-specific data that you might choose to record. For example, if a user rates a movie on your site, other than movie ID (<code>itemId</code>) and rating (<code>eventValue</code>) , you might also send the number of movie ratings made by the user.</p>
        /// <p>Each item in the map consists of a key-value pair. For example,</p>
        /// <p> <code>{"numberOfRatings": "12"}</code> </p>
        /// <p>The keys use camel case names that match the fields in the Interactions schema. In the above example, the <code>numberOfRatings</code> would match the 'NUMBER_OF_RATINGS' field defined in the Interactions schema.</p>
        pub fn properties(mut self, input: impl Into<std::string::String>) -> Self {
            self.properties = Some(input.into());
            self
        }
        /// <p>A string map of event-specific data that you might choose to record. For example, if a user rates a movie on your site, other than movie ID (<code>itemId</code>) and rating (<code>eventValue</code>) , you might also send the number of movie ratings made by the user.</p>
        /// <p>Each item in the map consists of a key-value pair. For example,</p>
        /// <p> <code>{"numberOfRatings": "12"}</code> </p>
        /// <p>The keys use camel case names that match the fields in the Interactions schema. In the above example, the <code>numberOfRatings</code> would match the 'NUMBER_OF_RATINGS' field defined in the Interactions schema.</p>
        pub fn set_properties(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.properties = input;
            self
        }
        /// <p>The timestamp (in Unix time) on the client side when the event occurred.</p>
        pub fn sent_at(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.sent_at = Some(input);
            self
        }
        /// <p>The timestamp (in Unix time) on the client side when the event occurred.</p>
        pub fn set_sent_at(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.sent_at = input;
            self
        }
        /// <p>The ID of the recommendation.</p>
        pub fn recommendation_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.recommendation_id = Some(input.into());
            self
        }
        /// <p>The ID of the recommendation.</p>
        pub fn set_recommendation_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.recommendation_id = input;
            self
        }
        /// Appends an item to `impression`.
        ///
        /// To override the contents of this collection use [`set_impression`](Self::set_impression).
        ///
        /// <p>A list of item IDs that represents the sequence of items you have shown the user. For example, <code>["itemId1", "itemId2", "itemId3"]</code>.</p>
        pub fn impression(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.impression.unwrap_or_default();
            v.push(input.into());
            self.impression = Some(v);
            self
        }
        /// <p>A list of item IDs that represents the sequence of items you have shown the user. For example, <code>["itemId1", "itemId2", "itemId3"]</code>.</p>
        pub fn set_impression(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.impression = input;
            self
        }
        /// Consumes the builder and constructs a [`Event`](crate::model::Event)
        pub fn build(self) -> crate::model::Event {
            crate::model::Event {
                event_id: self.event_id,
                event_type: self.event_type,
                event_value: self.event_value,
                item_id: self.item_id,
                properties: self.properties,
                sent_at: self.sent_at,
                recommendation_id: self.recommendation_id,
                impression: self.impression,
            }
        }
    }
}
impl Event {
    /// Creates a new builder-style object to manufacture [`Event`](crate::model::Event)
    pub fn builder() -> crate::model::event::Builder {
        crate::model::event::Builder::default()
    }
}
