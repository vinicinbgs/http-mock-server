extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream;

struct Route {
    pub method: String,
    pub name: String,
}

pub(crate) struct Router {
    routes: HashMap<String, Vec<String>>,
}

impl Router {
    pub fn add_route(&mut self, route: Route) {
        let routes = self.routes.entry(route.method).or_default();
        routes.push(route.name);
    }

    #[inline]
    pub fn routes(&self) -> &Self {
        self
    }
}

#[proc_macro_attribute]
pub fn route(
    attr: TokenStream, // GET, "/"
    item: TokenStream, // fn index() {}
) -> TokenStream {
    println!("{}", attr);
    println!("{}", item);
    Router::add_route(attr);
    item
}
