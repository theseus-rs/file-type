use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7654423572304751873: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "3gpp2 tcap",
    extensions: &["tcap"],
    media_types: &["application/vnd.3gpp2.tcap"],
    internal_signatures: &[],
    related_formats: &[],
};
