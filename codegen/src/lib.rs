use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro_error::*;
use quote::__private::TokenStream as QuoteTokenStream;
use quote::{format_ident, quote, ToTokens};
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    parse_macro_input, Data, DeriveInput, Error as SynError, Ident, Index, LitStr,
    Result as SynResult, Token,
};

#[derive(Clone)]
pub(crate) enum FieldRef {
    Ident(Ident),
    Index(Index),
}

impl ToTokens for FieldRef {
    fn to_tokens(&self, tokens: &mut QuoteTokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Index(index) => index.to_tokens(tokens),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum AnimationFieldType {
    Bool,
    Integer,
    Float,
    String,
}

struct AnimationFieldArgument {
    pub field: LitStr,
    pub ty: AnimationFieldType,
}

impl Parse for AnimationFieldArgument {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut field = None;
        let mut ty = None;

        while !input.is_empty() {
            let arg_name = input.parse::<Ident>()?;

            if arg_name == "field" {
                if field.is_some() {
                    return Err(SynError::new(input.span(), "duplicated argument: field"));
                }

                input.parse::<Token![=]>()?;
                let parsed = input.parse::<LitStr>()?;
                field = Some(parsed);

                if input.peek(Comma) {
                    input.parse::<Comma>()?;
                }
            } else if arg_name == "ty" {
                if ty.is_some() {
                    return Err(SynError::new(input.span(), "duplicated argument: ty"));
                }

                input.parse::<Token![=]>()?;
                let parsed = input.parse::<LitStr>()?;

                ty = Some(match parsed.value().as_str() {
                    "bool" => AnimationFieldType::Bool,
                    "integer" => AnimationFieldType::Integer,
                    "float" => AnimationFieldType::Float,
                    "string" => AnimationFieldType::String,
                    ty @ _ => {
                        return Err(SynError::new(
                            parsed.span(),
                            format!("invalid argument: type: {}", ty),
                        ));
                    }
                });

                if input.peek(Comma) {
                    input.parse::<Comma>()?;
                }
            } else {
                return Err(SynError::new(
                    input.span(),
                    format!("invalid argument: {}", arg_name),
                ));
            }
        }

        let field = if let Some(field) = field {
            field
        } else {
            return Err(SynError::new(input.span(), "missing argument: field"));
        };

        let ty = if let Some(ty) = ty {
            ty
        } else {
            return Err(SynError::new(input.span(), "missing argument: ty"));
        };

        Ok(Self { field, ty })
    }
}

#[proc_macro_derive(Animation, attributes(animate))]
#[proc_macro_error]
pub fn animation_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        return TokenStream::new();
    };

    let name = input.ident;
    let name_snake = name.to_string().to_case(Case::Snake);
    let mut field_index = 0;
    let mut field_set = HashSet::new();
    let mut fields = vec![];
    let mut animation_fields = vec![];
    let mut matching_tys = vec![];
    let mut matching_as_tys = vec![];

    for field in &data.fields {
        for attr in &field.attrs {
            if let Some(ident) = attr.path.get_ident() {
                if ident == "animate" {
                    let argument = attr
                        .parse_args::<AnimationFieldArgument>()
                        .unwrap_or_abort();
                    let argument_field = argument.field.value();

                    if field_set.contains(&argument_field) {
                        emit_error!(
                            argument.field.span(),
                            "duplicated field: {} in the struct: {}",
                            argument_field,
                            name
                        );
                    } else {
                        field_set.insert(argument_field);
                    }

                    match &field.ident {
                        Some(ident) => {
                            fields.push(FieldRef::Ident(ident.clone()));
                        }
                        None => {
                            let index = Index::from(field_index);
                            field_index += 1;
                            fields.push(FieldRef::Index(index));
                        }
                    };

                    animation_fields.push(argument.field);
                    matching_tys.push(format_ident!(
                        "{}",
                        match &argument.ty {
                            AnimationFieldType::Bool => "bool",
                            AnimationFieldType::Integer => "i64",
                            AnimationFieldType::Float => "f64",
                            AnimationFieldType::String => "String",
                        }
                    ));
                    matching_as_tys.push(format_ident!(
                        "as_{}",
                        match &argument.ty {
                            AnimationFieldType::Bool => "bool",
                            AnimationFieldType::Integer => "integer",
                            AnimationFieldType::Float => "float",
                            AnimationFieldType::String => "string",
                        }
                    ));
                }
            }
        }
    }

    let expanded = quote! {
        impl crate::component::Animate for #name {
            fn ty(&self) -> &'static str {
                #name_snake
            }

            fn animate(
                &mut self,
                time_line: &crate::animation::AnimationTimeLine,
                key_frame: &crate::animation::AnimationKeyFrame,
                normalized_time_in_key_frame: f32,
            ) {
                match time_line.field.as_str() {
                    #(
                        #animation_fields => {
                            self.#fields = <#matching_tys as crate::animation::Interpolatable>::interpolate(
                                key_frame.from.#matching_as_tys(),
                                key_frame.to.#matching_as_tys(),
                                normalized_time_in_key_frame,
                            ) as _;
                        }
                    )*
                    _ => {}
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(LuaComponent, attributes(lua_field, lua_hidden, lua_readonly))]
#[proc_macro_error]
pub fn lua_component_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let span = input.span();
    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        return TokenStream::new();
    };

    let name = input.ident;
    let type_name = format!("component:{}", name);
    let wrapper_name = format_ident!("LuaWrapper{}", name);
    let mut field_index = 0;
    let mut fields = vec![];
    let mut field_names = vec![];
    let mut non_readonly_fields = vec![];
    let mut non_readonly_field_names = vec![];
    let mut non_readonly_field_types = vec![];

    'field: for field in &data.fields {
        let mut field_name = None;
        let mut readonly = false;

        for attr in &field.attrs {
            if let Some(ident) = attr.path.get_ident() {
                if ident == "lua_hidden" {
                    continue 'field;
                }

                if ident == "lua_field" {
                    field_name = Some(format_ident!(
                        "{}",
                        attr.parse_args::<LitStr>().unwrap_or_abort().value()
                    ));
                }

                if ident == "lua_readonly" {
                    readonly = true;
                }
            }
        }

        let field_ref = match &field.ident {
            Some(ident) => FieldRef::Ident(ident.clone()),
            None => {
                let index = Index::from(field_index);
                field_index += 1;
                FieldRef::Index(index)
            }
        };
        let field_name = match field_name.as_ref() {
            Some(ident) => ident,
            None => match &field.ident {
                Some(ident) => ident,
                None => {
                    continue 'field;
                }
            },
        };

        fields.push(field_ref.clone());
        field_names.push(field_name.to_string());

        if !readonly {
            non_readonly_fields.push(field_ref);
            non_readonly_field_names.push(field_name.to_string());
            non_readonly_field_types.push(field.ty.clone());
        }
    }

    if fields.is_empty() {
        abort!(span, "the struct {} has no fields no expose", name);
    }

    let field_impls = quote! {
        methods.add_meta_method(
            mlua::MetaMethod::Index,
            |lua, this, index: String| match index.as_str() {
                "_type" => #type_name.to_lua(lua),
                #(
                    #field_names => {
                        let mut world = crate::api::use_context().world_mut();
                        let entry = match world.entry(this.0) {
                            Some(entry) => entry,
                            None => return Ok(mlua::Value::Nil),
                        };
                        let this = match entry.get_component::<#name>() {
                            Ok(this) => this,
                            Err(_) => return Ok(mlua::Value::Nil),
                        };
                        this.#fields.to_lua(lua)
                    }
                )*
                _ => Err(format!("the type {} has no such field {}", #type_name, index).to_lua_err()),
            },
        );
    };
    let non_readonly_field_impls = if non_readonly_fields.is_empty() {
        QuoteTokenStream::new()
    } else {
        quote! {
            methods.add_meta_method(
                mlua::MetaMethod::NewIndex,
                |lua, this, (index, value): (String, mlua::Value)| match index.as_str() {
                    #(
                        #non_readonly_field_names => {
                            let mut world = crate::api::use_context().world_mut();
                            let mut entry = match world.entry(this.0) {
                                Some(entry) => entry,
                                None => return Err(format!("the type {} used invalid entity id {:?}", #type_name, this.0).to_lua_err()),
                            };
                            let this = match entry.get_component_mut::<#name>() {
                                Ok(this) => this,
                                Err(_) => return Err(format!("the entity id {:?} does not contains the type {}", this.0, #type_name).to_lua_err()),
                            };
                            this.#non_readonly_fields = <#non_readonly_field_types as mlua::FromLua>::from_lua(value, lua)?;
                            Ok(())
                        }
                    )*
                    _ => Err(format!("the type {} has no such field {}", #type_name, index).to_lua_err()),
                },
            );
        }
    };

    let expanded = quote! {
        pub struct #wrapper_name(pub legion::Entity);

        impl From<legion::Entity> for #wrapper_name {
            fn from(entity: legion::Entity) -> Self {
                Self(entity)
            }
        }

        impl From<#wrapper_name> for legion::Entity {
            fn from(wrapper: #wrapper_name) -> Self {
                wrapper.0
            }
        }

        impl<'world> std::convert::TryFrom<&'world legion::world::Entry<'world>> for &'world #name {
            type Error = legion::world::ComponentError;

            fn try_from(entry: &'world legion::world::Entry) -> Result<Self, Self::Error> {
                entry.get_component()
            }
        }

        impl<'world> std::convert::TryFrom<&'world mut legion::world::Entry<'world>> for &'world mut #name {
            type Error = legion::world::ComponentError;

            fn try_from(entry: &'world mut legion::world::Entry) -> Result<Self, Self::Error> {
                entry.get_component_mut()
            }
        }

        impl mlua::UserData for #wrapper_name {
            fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
                use mlua::{ExternalError, ToLua};

                #field_impls
                #non_readonly_field_impls
            }
        }
    };

    TokenStream::from(expanded)
}
