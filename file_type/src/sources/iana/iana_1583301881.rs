use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1583301881: FileFormat = FileFormat {
    id: 1_583_301_881,
    source_type: SourceType::Iana,
    name: "vnd.intu.qfx",
    extensions: &[],
    media_types: &["application/vnd.intu.qfx"],
    internal_signatures: &[],
    related_formats: &[],
};
