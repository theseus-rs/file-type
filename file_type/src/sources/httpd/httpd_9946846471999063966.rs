use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9946846471999063966: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ace compressed",
    extensions: &["ace"],
    media_types: &["application/x-ace-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
