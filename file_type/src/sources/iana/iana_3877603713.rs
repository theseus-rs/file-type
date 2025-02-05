use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3877603713: FileFormat = FileFormat {
    id: 3_877_603_713,
    source_type: SourceType::Iana,
    name: "prs.implied-object+yaml",
    extensions: &[],
    media_types: &["application/prs.implied-object+yaml"],
    signatures: &[],
    related_formats: &[],
};
