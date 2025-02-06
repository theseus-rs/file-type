use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2565652218: FileFormat = FileFormat {
    id: 2_565_652_218,
    source_type: SourceType::Iana,
    name: "vnd.previewsystems.box",
    extensions: &[],
    media_types: &["application/vnd.previewsystems.box"],
    signatures: &[],
    related_formats: &[],
};
