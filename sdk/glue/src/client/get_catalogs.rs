// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetCatalogs`](crate::operation::get_catalogs::builders::GetCatalogsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`parent_catalog_id(impl Into<String>)`](crate::operation::get_catalogs::builders::GetCatalogsFluentBuilder::parent_catalog_id) / [`set_parent_catalog_id(Option<String>)`](crate::operation::get_catalogs::builders::GetCatalogsFluentBuilder::set_parent_catalog_id):<br>required: **false**<br><p>The ID of the parent catalog in which the catalog resides. If none is provided, the Amazon Web Services Account Number is used by default.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_catalogs::builders::GetCatalogsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_catalogs::builders::GetCatalogsFluentBuilder::set_next_token):<br>required: **false**<br><p>A continuation token, if this is a continuation call.</p><br>
    ///   - [`max_results(i32)`](crate::operation::get_catalogs::builders::GetCatalogsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_catalogs::builders::GetCatalogsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of catalogs to return in one response.</p><br>
    ///   - [`recursive(bool)`](crate::operation::get_catalogs::builders::GetCatalogsFluentBuilder::recursive) / [`set_recursive(Option<bool>)`](crate::operation::get_catalogs::builders::GetCatalogsFluentBuilder::set_recursive):<br>required: **false**<br><p>When specified as true, iterates through the account and returns all catalog resources (including top-level resources and child resources)</p><br>
    /// - On success, responds with [`GetCatalogsOutput`](crate::operation::get_catalogs::GetCatalogsOutput) with field(s):
    ///   - [`catalog_list(Vec::<Catalog>)`](crate::operation::get_catalogs::GetCatalogsOutput::catalog_list): <p>An array of <code>Catalog</code> objects. A list of <code>Catalog</code> objects from the specified parent catalog.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_catalogs::GetCatalogsOutput::next_token): <p>A continuation token for paginating the returned list of tokens, returned if the current segment of the list is not the last.</p>
    /// - On failure, responds with [`SdkError<GetCatalogsError>`](crate::operation::get_catalogs::GetCatalogsError)
    pub fn get_catalogs(&self) -> crate::operation::get_catalogs::builders::GetCatalogsFluentBuilder {
        crate::operation::get_catalogs::builders::GetCatalogsFluentBuilder::new(self.handle.clone())
    }
}
