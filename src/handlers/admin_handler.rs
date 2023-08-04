use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;

use crate::avored_state::AvoRedState;

pub async fn admin_handler(state: State<Arc<AvoRedState>>) -> impl IntoResponse {
    // let datastore = &state.datastore;
    // let database_session = &state.database_session;

    // let sql = "DELETE admin_users where name = 'Purvesh';";

    // let _responses = match datastore.execute(sql, &database_session, None, false).await {
    //     Ok(response) => response,
    //     Err(_) => {
    //         // todo improve this error
    //         let out: Vec<Response> = vec![];
    //         out
    //     }
    // };

    // let sql = "CREATE admin_users SET name = 'Purvesh';";

    // let _responses = match datastore.execute(sql, &database_session, None, false).await {
    //     Ok(response) => response,
    //     Err(_) => {
    //         // todo improve this error
    //         let out: Vec<Response> = vec![];
    //         out
    //     }
    // };
    // let sql = "SELECT * FROM admin_users;";

    // let responses = match datastore.execute(sql, &database_session, None, false).await {
    //     Ok(response) => response,
    //     Err(_) => {
    //         // todo improve this error
    //         let out: Vec<Response> = vec![];
    //         out
    //     }
    // };

    // let response = responses
    //     .into_iter()
    //     .next()
    //     .expect("error while retriving the first response");

    // let result = response.result.expect("first result comes with error");

    // let array: Array = W(result).try_into().expect("sdfds");
    // let objects: Result<Vec<Object>> = array.into_iter().map(|value| W(value).try_into()).collect();
    // let objects = match objects {
    //     Ok(obj) => obj,
    //     Err(_) => panic!("no data"),
    // };

    // let test: Result<Vec<AdminUser>> = objects.into_iter().map(|o| o.try_into()).collect();

    // let admin_users = match test {
    //     Ok(data) => data,
    //     Err(_) => panic!("some errror"),
    // };

    let admin_users = state
        .admin_user_repository
        .paginate(&state.datastore, &state.database_session)
        .await;

     let admin_users = match admin_users {
        Ok(data) => data,
        Err(_) => panic!("no data found error"),
    };

    Json(admin_users).into_response()
}
