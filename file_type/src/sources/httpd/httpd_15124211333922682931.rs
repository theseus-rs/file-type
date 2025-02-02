use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15124211333922682931: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "zul",
    extensions: &["zir", "zirz"],
    media_types: &["application/vnd.zul"],
    internal_signatures: &[],
    related_formats: &[],
};
