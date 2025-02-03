use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_975017874: FileFormat = FileFormat {
    id: 975_017_874,
    source_type: SourceType::Iana,
    name: "vnd.sealed.csf",
    extensions: &[],
    media_types: &["application/vnd.sealed.csf"],
    internal_signatures: &[],
    related_formats: &[],
};
