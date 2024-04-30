use crate::db::schema::links;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Link {
    pub id: Uuid,
    pub target_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Link {
    pub fn into_proto(&self) -> crate::shortener::Link {
        crate::shortener::Link {
            id: self.id.to_string(),
            target_url: self.target_url.clone(),
            created_at: self.created_at.to_string(),
            updated_at: match self.updated_at {
                Some(val) => Some(val.to_string()),
                None => None,
            },
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = links)]
pub struct NewLink<'a> {
    pub target_url: &'a str,
}

// Create a new link in the database
pub fn create_link(conn: &mut PgConnection, target_url: &str) -> Link {
    let new_link = NewLink { target_url };

    diesel::insert_into(links::table)
        .values(&new_link)
        .returning(Link::as_returning())
        .get_result(conn)
        .expect("Error saving new link")
}

// Get a link by it's target url in the database
pub fn get_link_by_url(conn: &mut PgConnection, by_url: &str) -> Option<Link> {
    use crate::db::schema::links::dsl::*;

    let link = links
        .filter(target_url.eq(by_url))
        .select(Link::as_select())
        .first(conn)
        .optional();

    match link {
        Ok(Some(link)) => Some(link),
        Ok(None) => None,
        Err(_) => {
            println!("An error occurred while fetching link by url {:?}", by_url);
            None
        }
    }
}

// Get a link by it's primary key
pub fn get_link_by_id(conn: &mut PgConnection, link_id: Uuid) -> Option<Link> {
    use crate::db::schema::links::dsl::links;

    let link = links
        .find(link_id)
        .select(Link::as_select())
        .first(conn)
        .optional();

    match link {
        Ok(Some(found_link)) => Some(found_link),
        Ok(None) => None,
        Err(_) => {
            println!(
                "An error occurred while fetching a link by id {:?}",
                link_id
            );
            None
        }
    }
}
