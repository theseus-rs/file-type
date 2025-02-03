use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3827382622: FileFormat = FileFormat {
    id: 3_827_382_622,
    source_type: SourceType::Httpd,
    name: "mac binhex40",
    extensions: &["hqx"],
    media_types: &["application/mac-binhex40"],
    internal_signatures: &[],
    related_formats: &[],
};
