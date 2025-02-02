use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5785170104885681682: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "g3fax",
    extensions: &["g3"],
    media_types: &["image/g3fax"],
    internal_signatures: &[],
    related_formats: &[],
};
