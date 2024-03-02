use crate::{
    connect::DbConnect as Connect, entity::_pagination::Paginate, schema::system_menus, Error,
    Result,
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::{scoped_futures::*, AsyncConnection, RunQueryDsl, SaveChangesDsl};
use getset::Getters;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::time::SystemTime;

/// define Entity
#[derive(Debug, Queryable, Selectable, Identifiable, AsChangeset, Serialize, Getters)]
#[diesel(table_name = crate::schema::system_menus)]
pub struct Entity {
    #[getset(get = "pub")]
    id: i32,
    /// 父级ID
    parent_id: i32,
    /// 菜单类型:1.菜单,2.重定向/目录,3.外链,4.嵌套,5.按钮权限,6.接口权限
    #[diesel(column_name = "type_")]
    r#type: i32,
    /// 菜单名称
    title: String,
    /// 图标
    icon: String,
    /// 路由名称
    router_name: String,
    /// 组件地址
    router_component: String,
    /// 路径
    router_path: String,
    /// 重定向
    redirect: String,
    /// 外链地址
    link: String,
    /// 内嵌地址
    iframe: String,
    /// 按钮权限
    btn_auth: String,
    /// 接口地址
    api_url: String,
    /// 接口请求方法
    api_method: String,
    /// 是否隐藏
    is_hide: i32,
    /// 是否开启keep_alive
    is_keep_alive: i32,
    /// 是否固定
    is_affix: i32,
    /// 排序
    sort: i32,
    created_at: SystemTime,
    updated_at: SystemTime,
    deleted_at: Option<SystemTime>,
}

/// impl Entity method
impl Entity {
    /// query find
    pub async fn find(conn: &mut Connect, filter: &Filter) -> Result<Option<Self>> {
        let table = system_menus::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_menus::name.eq(_keyword));
        }

        let info = table
            .select(Entity::as_select())
            .first::<Entity>(conn)
            .await
            .optional()?;
        Ok(info)
    }
    /// query method
    pub async fn query(conn: &mut Connect, filter: &Filter) -> Result<Vec<Self>> {
        let table = system_menus::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_menus::name.eq(_keyword));
        }

        let infos = table
            .select(Entity::as_select())
            .load::<Entity>(conn)
            .await?;
        Ok(infos)
    }
    /// paginate method
    pub async fn paginate(
        conn: &mut Connect,
        page: i64,
        limit: i64,
        filter: &Filter,
    ) -> Result<(Vec<Self>, i64)> {
        let table = system_menus::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_menus::name.eq(_keyword));
        }

        Ok(table
            .select(Entity::as_select())
            .paginate(page)
            .per_page(limit)
            .load_and_count_pages::<Entity>(conn)
            .await?)
    }
    /// insert method
    pub async fn insert(conn: &mut Connect, params: Vec<FormParamsForCreate>) -> Result<Vec<Self>> {
        Ok(insert_into(system_menus::dsl::system_menus)
            .values(params)
            .get_results(conn)
            .await?)
    }
    /// create method
    pub async fn create(conn: &mut Connect, param: &FormParamsForCreate) -> Result<Self> {
        Ok(insert_into(system_menus::dsl::system_menus)
            .values(param)
            .get_result(conn)
            .await?)
    }
    /// update mthod
    pub async fn update(conn: &mut Connect, id: i32, params: FormParamsForCreate) -> Result<Self> {
        Ok(
            update(system_menus::dsl::system_menus.filter(system_menus::id.eq(id)))
                .set(params)
                .get_result(conn)
                .await?,
        )
    }
    /// soft_delete method
    pub async fn soft_delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            update(system_menus::dsl::system_menus.filter(system_menus::id.eq(id)))
                .set(system_menus::deleted_at.eq(Some(SystemTime::now())))
                .get_result(conn)
                .await?,
        )
    }
    /// delete method
    pub async fn delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            delete(system_menus::dsl::system_menus.filter(system_menus::id.eq(id)))
                .get_result(conn)
                .await?,
        )
    }
    /// soft_delete_transaction method
    pub async fn soft_delete_transaction(conn: &mut Connect, id: i32) -> Result<Self> {
        let info = conn
            .transaction::<_, Error, _>(|conn| {
                async move {
                    let info = Self::soft_delete(conn, id).await?;
                    Ok(info)
                    // other action
                }
                .scope_boxed()
            })
            .await?;
        Ok(info)
    }
    // others methods
}
/// define Filter
#[derive(Debug, Default, Deserialize)]
pub struct Filter {
    pub keyword: Option<String>,
    pub status: Option<i32>,
    // other fields
    pub ids: Option<Vec<i32>>,
}
/// define Forms Param
#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::system_menus)]
pub struct FormParamsForCreate {
    parent_id: i32,
    #[diesel(column_name = "type_")]
    r#type: i32,
    title: String,
    icon: String,
    router_name: String,
    router_component: String,
    router_path: String,
    redirect: String,
    link: String,
    iframe: String,
    btn_auth: String,
    api_url: String,
    api_method: String,
    is_hide: i32,
    is_keep_alive: i32,
    is_affix: i32,
    sort: i32,
}

pub type FormParamsForUpdate = FormParamsForCreate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MenuType {
    /// 1.菜单
    Menu = 1,
    /// 2.重定向/目录
    Redirect = 2,
    /// 3.外链
    Link = 3,
    /// 4.嵌套
    Iframe = 4,
    /// 5.按钮权限
    BtnAuth = 5,
    /// 6.接口权限
    Api = 6,
}
