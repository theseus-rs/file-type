use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3709208966: FileFormat = FileFormat {
    id: 3_709_208_966,
    source_type: SourceType::Iana,
    name: "vnd.dece.pd",
    extensions: &[],
    media_types: &["video/vnd.dece.pd"],
    internal_signatures: &[],
    related_formats: &[],
};
