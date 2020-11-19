use anyhow::{anyhow, Result};
use syn::{Attribute, MetaList};

pub fn process_directive<T>(
    attrs: &[Attribute],
    mut f: impl FnMut(&MetaList) -> Result<T>,
    attr_name: &str,
    required: bool,
) -> Result<()> {
    let mut seen = 0;

    for attr in attrs {
        let meta = attr.parse_meta()?;
        match meta {
            syn::Meta::List(list) => {
                match list.path.get_ident() {
                    Some(path) if path.to_string() == attr_name => {
                        seen += 1;
                        f(&list)?;
                    }
                    _ => {
                        // ignore
                    }
                }
            }
            _ => {
                // ignore
            }
        }
    }

    if required && seen == 0 {
        return Err(anyhow!("missing required attribute {}", attr_name));
    }

    Ok(())
}
