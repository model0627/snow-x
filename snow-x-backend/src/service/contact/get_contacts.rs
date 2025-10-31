use crate::dto::contact::response::{ContactInfoResponse, ContactListResponse};
use crate::entity::contacts;
use crate::service::error::errors::ServiceResult;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};

pub async fn service_get_contacts(
    conn: &DatabaseConnection,
    page: u64,
    limit: u64,
    search: Option<String>,
    department: Option<String>,
    is_active: Option<bool>,
) -> ServiceResult<ContactListResponse> {
    let mut query = contacts::Entity::find();

    // Apply is_active filter (defaults to true if not specified)
    let active_filter = is_active.unwrap_or(true);
    query = query.filter(contacts::Column::IsActive.eq(active_filter));

    // Apply search filter
    if let Some(search_term) = search {
        query = query.filter(
            contacts::Column::Name
                .contains(&search_term)
                .or(contacts::Column::Email.contains(&search_term))
                .or(contacts::Column::Phone.contains(&search_term))
                .or(contacts::Column::Mobile.contains(&search_term)),
        );
    }

    // Apply department filter
    if let Some(dept) = department {
        query = query.filter(contacts::Column::Department.contains(&dept));
    }

    let paginator = query
        .order_by_desc(contacts::Column::CreatedAt)
        .paginate(conn, limit);

    let total = paginator.num_items().await?;
    let contacts_list = paginator.fetch_page(page - 1).await?;

    let contact_responses: Vec<ContactInfoResponse> = contacts_list
        .into_iter()
        .map(|contact| ContactInfoResponse {
            id: contact.id,
            name: contact.name,
            title: contact.title,
            department: contact.department,
            phone: contact.phone,
            mobile: contact.mobile,
            email: contact.email,
            office_location: contact.office_location,
            responsibilities: contact.responsibilities,
            created_by: contact.created_by,
            created_at: contact.created_at.to_string(),
            updated_at: contact.updated_at.to_string(),
            is_active: contact.is_active,
            source_type: contact.source_type,
            external_api_connection_id: contact.external_api_connection_id,
        })
        .collect();

    Ok(ContactListResponse {
        contacts: contact_responses,
        total,
        page,
        limit,
    })
}
