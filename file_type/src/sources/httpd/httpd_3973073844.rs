use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3973073844: FileFormat = FileFormat {
    id: 3_973_073_844,
    source_type: SourceType::Httpd,
    name: "ace compressed",
    extensions: &["ace"],
    media_types: &["application/x-ace-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
