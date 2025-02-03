use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4273659658: FileFormat = FileFormat {
    id: 4_273_659_658,
    source_type: SourceType::Iana,
    name: "ST2110-41",
    extensions: &[],
    media_types: &["application/ST2110-41"],
    internal_signatures: &[],
    related_formats: &[],
};
