use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3128901905: FileFormat = FileFormat {
    id: 3_128_901_905,
    source_type: SourceType::Iana,
    name: "vnd.efi.img",
    extensions: &[],
    media_types: &["application/vnd.efi.img"],
    signatures: &[],
    related_formats: &[],
};
