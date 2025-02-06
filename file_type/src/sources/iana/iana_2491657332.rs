use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2491657332: FileFormat = FileFormat {
    id: 2_491_657_332,
    source_type: SourceType::Iana,
    name: "beep+xml",
    extensions: &[],
    media_types: &["application/beep+xml"],
    signatures: &[],
    related_formats: &[],
};
