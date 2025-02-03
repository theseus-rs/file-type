use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1365633832: FileFormat = FileFormat {
    id: 1_365_633_832,
    source_type: SourceType::Iana,
    name: "vnd.abc",
    extensions: &[],
    media_types: &["text/vnd.abc"],
    internal_signatures: &[],
    related_formats: &[],
};
