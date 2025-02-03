use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_276593833: FileFormat = FileFormat {
    id: 276_593_833,
    source_type: SourceType::Iana,
    name: "vnd.truedoc",
    extensions: &[],
    media_types: &["application/vnd.truedoc"],
    internal_signatures: &[],
    related_formats: &[],
};
