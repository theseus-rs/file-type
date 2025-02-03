use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1514583043: FileFormat = FileFormat {
    id: 1_514_583_043,
    source_type: SourceType::Iana,
    name: "vnd.nintendo.snes.rom",
    extensions: &[],
    media_types: &["application/vnd.nintendo.snes.rom"],
    internal_signatures: &[],
    related_formats: &[],
};
