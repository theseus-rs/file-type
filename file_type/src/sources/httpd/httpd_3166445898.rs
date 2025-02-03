use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3166445898: FileFormat = FileFormat {
    id: 3_166_445_898,
    source_type: SourceType::Httpd,
    name: "mesh",
    extensions: &["msh", "mesh", "silo"],
    media_types: &["model/mesh"],
    internal_signatures: &[],
    related_formats: &[],
};
