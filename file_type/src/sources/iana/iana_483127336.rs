use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_483127336: FileFormat = FileFormat {
    id: 483_127_336,
    source_type: SourceType::Iana,
    name: "mpeg",
    extensions: &[],
    media_types: &["audio/mpeg"],
    internal_signatures: &[],
    related_formats: &[],
};
