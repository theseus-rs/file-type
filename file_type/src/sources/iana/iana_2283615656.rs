use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2283615656: FileFormat = FileFormat {
    id: 2_283_615_656,
    source_type: SourceType::Iana,
    name: "vnd.nuera.ecelp4800",
    extensions: &[],
    media_types: &["audio/vnd.nuera.ecelp4800"],
    internal_signatures: &[],
    related_formats: &[],
};
