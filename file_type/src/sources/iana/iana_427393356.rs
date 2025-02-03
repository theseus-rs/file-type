use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_427393356: FileFormat = FileFormat {
    id: 427_393_356,
    source_type: SourceType::Iana,
    name: "vnd.unity",
    extensions: &[],
    media_types: &["application/vnd.unity"],
    internal_signatures: &[],
    related_formats: &[],
};
