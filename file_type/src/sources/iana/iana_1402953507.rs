use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1402953507: FileFormat = FileFormat {
    id: 1_402_953_507,
    source_type: SourceType::Iana,
    name: "amr-wb+",
    extensions: &[],
    media_types: &["audio/amr-wb+"],
    signatures: &[],
    related_formats: &[],
};
