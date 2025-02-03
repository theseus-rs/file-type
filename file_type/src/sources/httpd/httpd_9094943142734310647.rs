use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9094943142734310647: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mobius msl",
    extensions: &["msl"],
    media_types: &["application/vnd.mobius.msl"],
    internal_signatures: &[],
    related_formats: &[],
};
