// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`ListPlacements`](crate::operation::ListPlacements)
pub struct ListPlacementsPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_placements_input::Builder,
}

impl<C, M, R> ListPlacementsPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_placements_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `placements`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::ListPlacementsPaginatorItems<C, M, R> {
        crate::paginator::ListPlacementsPaginatorItems(self)
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListPlacementsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListPlacementsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListPlacementsInputOperationOutputAlias,
            crate::output::ListPlacementsOutput,
            crate::error::ListPlacementsError,
            crate::input::ListPlacementsInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            input.next_token = crate::lens::reflens_structure_crate_output_list_placements_output_next_token(resp).cloned();
                            input.next_token.is_none()
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListProjects`](crate::operation::ListProjects)
pub struct ListProjectsPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_projects_input::Builder,
}

impl<C, M, R> ListProjectsPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_projects_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `projects`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::ListProjectsPaginatorItems<C, M, R> {
        crate::paginator::ListProjectsPaginatorItems(self)
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListProjectsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListProjectsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListProjectsInputOperationOutputAlias,
            crate::output::ListProjectsOutput,
            crate::error::ListProjectsError,
            crate::input::ListProjectsInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            input.next_token = crate::lens::reflens_structure_crate_output_list_projects_output_next_token(resp).cloned();
                            input.next_token.is_none()
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Flattened paginator for `ListPlacementsPaginator`
///
/// This is created with [`.items()`](ListPlacementsPaginator::items)
pub struct ListPlacementsPaginatorItems<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
>(ListPlacementsPaginator<C, M, R>);

impl<C, M, R> ListPlacementsPaginatorItems<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::PlacementSummary,
            aws_smithy_http::result::SdkError<crate::error::ListPlacementsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListPlacementsInputOperationOutputAlias,
            crate::output::ListPlacementsOutput,
            crate::error::ListPlacementsError,
            crate::input::ListPlacementsInputOperationRetryAlias,
        >,
    {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
            crate::lens::lens_structure_crate_output_list_placements_output_placements(page)
                .unwrap_or_default()
                .into_iter()
        })
    }
}

/// Flattened paginator for `ListProjectsPaginator`
///
/// This is created with [`.items()`](ListProjectsPaginator::items)
pub struct ListProjectsPaginatorItems<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
>(ListProjectsPaginator<C, M, R>);

impl<C, M, R> ListProjectsPaginatorItems<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::ProjectSummary,
            aws_smithy_http::result::SdkError<crate::error::ListProjectsError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListProjectsInputOperationOutputAlias,
            crate::output::ListProjectsOutput,
            crate::error::ListProjectsError,
            crate::input::ListProjectsInputOperationRetryAlias,
        >,
    {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
            crate::lens::lens_structure_crate_output_list_projects_output_projects(page)
                .unwrap_or_default()
                .into_iter()
        })
    }
}
