use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_712815207: FileFormat = FileFormat {
    id: 712_815_207,
    source_type: SourceType::Iana,
    name: "efi",
    extensions: &[],
    media_types: &["application/efi"],
    signatures: &[],
    related_formats: &[],
};
