use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1207355911: FileFormat = FileFormat {
    id: 1_207_355_911,
    source_type: SourceType::Iana,
    name: "vnd.multiad.creator",
    extensions: &[],
    media_types: &["application/vnd.multiad.creator"],
    signatures: &[],
    related_formats: &[],
};
