use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16114080177138872421: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mbox",
    extensions: &["mbox"],
    media_types: &["application/mbox"],
    internal_signatures: &[],
    related_formats: &[],
};
