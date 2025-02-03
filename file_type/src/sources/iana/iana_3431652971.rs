use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3431652971: FileFormat = FileFormat {
    id: 3_431_652_971,
    source_type: SourceType::Iana,
    name: "vnd.nuera.ecelp7470",
    extensions: &[],
    media_types: &["audio/vnd.nuera.ecelp7470"],
    internal_signatures: &[],
    related_formats: &[],
};
