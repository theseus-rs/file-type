use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1162461536: FileFormat = FileFormat {
    id: 1_162_461_536,
    source_type: SourceType::Iana,
    name: "vnd.airzip.accelerator.azv",
    extensions: &[],
    media_types: &["image/vnd.airzip.accelerator.azv"],
    internal_signatures: &[],
    related_formats: &[],
};
