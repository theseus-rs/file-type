use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2563513024: FileFormat = FileFormat {
    id: 2_563_513_024,
    source_type: SourceType::Iana,
    name: "p21+zip",
    extensions: &[],
    media_types: &["application/p21+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
