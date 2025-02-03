use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6674260812445752722: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mesh",
    extensions: &["msh", "mesh", "silo"],
    media_types: &["model/mesh"],
    internal_signatures: &[],
    related_formats: &[],
};
