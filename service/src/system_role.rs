use crate::{Result, ServiceError};
use model::{connect::DbConnectPool as ConnectPool, system_role, system_role_menu};
use serde::{Deserialize, Serialize};
use utils::paginate::{PaginateParams, PaginateResult};

pub async fn create(
    pool: &ConnectPool,
    params: system_role::FormParamsForCreate,
    menu_ids: Vec<i32>,
) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_role::Entity::create_with_menus(&mut conn, &params, menu_ids).await?)
}

pub async fn update(
    pool: &ConnectPool,
    id: i32,
    params: system_role::FormParamsForCreate,
    menu_ids: Vec<i32>,
) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_role::Entity::update_with_menus(&mut conn, id, params, menu_ids).await?)
}

pub async fn delete(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    let result = system_role::Entity::soft_delete_transaction(&mut conn, id).await?;
    Ok(result)
}

pub async fn all(pool: &ConnectPool) -> Result<Vec<Info>> {
    let mut conn = pool.conn().await?;
    let infos = system_role::Entity::query(
        &mut conn,
        &system_role::Filter {
            ..Default::default()
        },
    )
    .await?;
    Ok(infos)
}

pub async fn paginate(pool: &ConnectPool, filter: &Filter) -> Result<PaginateResult<Vec<Info>>> {
    let mut conn = pool.conn().await?;
    let (data, total) = system_role::Entity::paginate(
        &mut conn,
        filter.paginate.get_page(),
        filter.paginate.get_limit(),
        &filter.filter,
    )
    .await?;
    Ok(PaginateResult { total, data })
}

pub async fn info(pool: &ConnectPool, id: i32) -> Result<InfoWithMenuIds> {
    let mut conn = pool.conn().await?;
    let mut info: InfoWithMenuIds = system_role::Entity::find(
        &mut conn,
        &system_role::Filter {
            id: Some(id),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)?
    .into();
    info.menu_ids = system_role_menu::Entity::get_menu_ids_by_role_id(&mut conn, id).await?;
    Ok(info)
}

pub async fn get_by_sign(pool: &ConnectPool, sign: &str, id: Option<i32>) -> Result<Option<Info>> {
    let mut conn = pool.conn().await?;
    let role_result = system_role::Entity::find(
        &mut conn,
        &system_role::Filter {
            sign: Some(sign.to_owned()),
            ..Default::default()
        },
    )
    .await?;
    Ok(match (id, role_result) {
        (Some(id), Some(role)) if role.id.ne(&id) => Some(role),
        _ => None,
    })
}
pub type Info = system_role::Entity;

#[derive(Debug, Serialize)]
pub struct InfoWithMenuIds {
    #[serde(flatten)]
    info: system_role::Entity,
    menu_ids: Vec<i32>,
}
impl From<system_role::Entity> for InfoWithMenuIds {
    fn from(data: system_role::Entity) -> Self {
        Self {
            info: data,
            menu_ids: vec![],
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Filter {
    #[serde(flatten)]
    filter: system_role::Filter,
    #[serde(flatten)]
    paginate: PaginateParams,
}
