use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2699770000: FileFormat = FileFormat {
    id: 2_699_770_000,
    source_type: SourceType::Iana,
    name: "vnd.omaloc-supl-init",
    extensions: &[],
    media_types: &["application/vnd.omaloc-supl-init"],
    signatures: &[],
    related_formats: &[],
};
