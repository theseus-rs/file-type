use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_913650795: FileFormat = FileFormat {
    id: 913_650_795,
    source_type: SourceType::Iana,
    name: "vnd.relpipe",
    extensions: &[],
    media_types: &["application/vnd.relpipe"],
    internal_signatures: &[],
    related_formats: &[],
};
