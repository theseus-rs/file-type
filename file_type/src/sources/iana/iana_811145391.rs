use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_811145391: FileFormat = FileFormat {
    id: 811_145_391,
    source_type: SourceType::Iana,
    name: "dsr-es202212",
    extensions: &[],
    media_types: &["audio/dsr-es202212"],
    internal_signatures: &[],
    related_formats: &[],
};
