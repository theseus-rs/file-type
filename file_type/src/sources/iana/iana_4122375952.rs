use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4122375952: FileFormat = FileFormat {
    id: 4_122_375_952,
    source_type: SourceType::Iana,
    name: "DV",
    extensions: &[],
    media_types: &["audio/DV"],
    internal_signatures: &[],
    related_formats: &[],
};
