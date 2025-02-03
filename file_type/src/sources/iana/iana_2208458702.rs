use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2208458702: FileFormat = FileFormat {
    id: 2_208_458_702,
    source_type: SourceType::Iana,
    name: "urc-grpsheet+xml",
    extensions: &[],
    media_types: &["application/urc-grpsheet+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
