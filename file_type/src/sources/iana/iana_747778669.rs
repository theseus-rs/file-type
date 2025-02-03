use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_747778669: FileFormat = FileFormat {
    id: 747_778_669,
    source_type: SourceType::Iana,
    name: "vnd.visionary",
    extensions: &[],
    media_types: &["application/vnd.visionary"],
    internal_signatures: &[],
    related_formats: &[],
};
