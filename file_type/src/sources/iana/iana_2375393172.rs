use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2375393172: FileFormat = FileFormat {
    id: 2_375_393_172,
    source_type: SourceType::Iana,
    name: "vnd.presonus.multitrack",
    extensions: &[],
    media_types: &["audio/vnd.presonus.multitrack"],
    internal_signatures: &[],
    related_formats: &[],
};
