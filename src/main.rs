use anyhow::Context;
use clap::Parser as _;
use rayon::prelude::*;
use std::{
    fmt::{self, Display},
    fs,
    path::PathBuf,
};

#[derive(clap::Parser)]
struct Args {
    paths: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    args.paths.par_iter().try_for_each(|path| {
        let source = fs::read_to_string(path)
            .with_context(|| format!("failed to read '{}'", path.display()))?;

        let ast = syn::parse_file(&source)
            .with_context(|| format!("failed to parse '{}' as Rust code", path.display()))?;

        for syn_item in ast.items {
            if let Ok(mut toc_item) = TocItem::try_from(&syn_item) {
                toc_item.path = Some(path.clone());
                println!("{toc_item}");
            }
        }

        Ok(())
    })
}

struct TocItem<'ast> {
    path: Option<PathBuf>,
    line: usize,
    column: usize,
    token: &'static str,
    ident: &'ast syn::Ident,
}

impl<'ast> TocItem<'ast> {
    fn new(token: &'static str, ident: &'ast syn::Ident) -> Self {
        let span = ident.span();
        let line = span.start().line;
        let column = span.start().column + 1;
        Self {
            path: None,
            line,
            column,
            token,
            ident,
        }
    }
}

impl<'ast> TryFrom<&'ast syn::Item> for TocItem<'ast> {
    type Error = String;

    fn try_from(item: &'ast syn::Item) -> Result<Self, Self::Error> {
        match item {
            syn::Item::Const(item_const) => Ok(TocItem::new("const", &item_const.ident)),
            syn::Item::Enum(item_enum) => Ok(TocItem::new("enum", &item_enum.ident)),
            syn::Item::Fn(item_fn) => Ok(TocItem::new("fn", &item_fn.sig.ident)),
            syn::Item::Static(item_static) => Ok(TocItem::new("static", &item_static.ident)),
            syn::Item::Struct(item_struct) => Ok(TocItem::new("struct", &item_struct.ident)),
            syn::Item::Trait(item_trait) => Ok(TocItem::new("trait", &item_trait.ident)),
            syn::Item::TraitAlias(item_trait_alias) => {
                Ok(TocItem::new("trait", &item_trait_alias.ident))
            }
            syn::Item::Type(item_type) => Ok(TocItem::new("type", &item_type.ident)),
            syn::Item::Union(item_union) => Ok(TocItem::new("union", &item_union.ident)),
            syn::Item::ExternCrate(_item_extern_crate) => {
                Err(String::from("unsupported item: ExternCrate"))
            }
            syn::Item::ForeignMod(_item_foreign_mod) => {
                Err(String::from("unsupported item: ForeignMod"))
            }
            syn::Item::Impl(_item_impl) => Err(String::from("unsupported item: Impl")),
            syn::Item::Macro(_item_macro) => Err(String::from("unsupported item: Macro")),
            syn::Item::Mod(_item_mod) => Err(String::from("unsupported item: Mod")),
            syn::Item::Use(_item_use) => Err(String::from("unsupported item: Use")),
            syn::Item::Verbatim(_token_stream) => Err(String::from("unsupported item: Verbatim")),
            other => Err(format!("unknown item: {other:?}")),
        }
    }
}

impl Display for TocItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if let Some(path) = &self.path {
            write!(
                f,
                "{}:{}:{}:{} {}",
                path.display(),
                self.line,
                self.column,
                self.token,
                self.ident
            )
        } else {
            write!(
                f,
                "<unknown path>:{}:{}:{} {}",
                self.line, self.column, self.token, self.ident
            )
        }
    }
}
