pub mod api;
pub mod components;
pub mod paths;

pub trait Printable {
    fn print(&self) -> proc_macro2::TokenStream;
}

impl<T> Printable for Vec<T>
where
    T: Printable,
{
    fn print(&self) -> proc_macro2::TokenStream {
        use quote::quote;

        let mut stream = quote! {};

        for field in self {
            let printed = field.print();

            stream = quote! {
                #stream
                #printed
            };
        }

        stream
    }
}

pub struct GeneratedModule {
    pub api_module: api::module::ApiModule,
    pub components_module: components::module::ComponentsModule,
    pub paths_module: paths::module::PathsModule,
}

impl Printable for GeneratedModule {
    fn print(&self) -> proc_macro2::TokenStream {
        let api_module = self.api_module.print();
        let components_module = self.components_module.print();
        let paths_module = self.paths_module.print();

        quote::quote! {
            #api_module
            #components_module
            #paths_module
        }
    }
}
