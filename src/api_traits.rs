use datatypes::api_types::Route;

pub trait DatabaseHandler {
    fn get_one_by_id<T>(id: String) -> T;
    fn delete_one_by_id<T>(id: String) -> T;
    fn update_one_by_id<T>(id: String, data: T) -> T;
    fn create_one_by_id<T>(id: String, data: T) -> T;
}

pub struct RouteHandlerData {
    routes: Vec<Route>,
}

pub trait RouteHandler {
    fn create_route(route: Route) -> ();
}
