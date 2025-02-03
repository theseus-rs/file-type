use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13793382431961437112: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms xpsdocument",
    extensions: &["xps"],
    media_types: &["application/vnd.ms-xpsdocument"],
    internal_signatures: &[],
    related_formats: &[],
};
