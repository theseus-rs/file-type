use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4276800422078512688: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cmdf",
    extensions: &["cmdf"],
    media_types: &["chemical/x-cmdf"],
    internal_signatures: &[],
    related_formats: &[],
};
