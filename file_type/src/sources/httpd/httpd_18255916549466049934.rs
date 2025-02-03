use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_18255916549466049934: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "amiga ami",
    extensions: &["ami"],
    media_types: &["application/vnd.amiga.ami"],
    internal_signatures: &[],
    related_formats: &[],
};
