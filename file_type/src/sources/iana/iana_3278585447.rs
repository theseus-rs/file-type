use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3278585447: FileFormat = FileFormat {
    id: 3_278_585_447,
    source_type: SourceType::Iana,
    name: "vnd.groove-vcard",
    extensions: &[],
    media_types: &["application/vnd.groove-vcard"],
    internal_signatures: &[],
    related_formats: &[],
};
