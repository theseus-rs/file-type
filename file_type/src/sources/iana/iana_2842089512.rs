use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2842089512: FileFormat = FileFormat {
    id: 2_842_089_512,
    source_type: SourceType::Iana,
    name: "vnd.nintendo.nitro.rom",
    extensions: &[],
    media_types: &["application/vnd.nintendo.nitro.rom"],
    internal_signatures: &[],
    related_formats: &[],
};
