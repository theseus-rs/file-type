use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_348215381: FileFormat = FileFormat {
    id: 348_215_381,
    source_type: SourceType::Iana,
    name: "vnd.kde.kchart",
    extensions: &[],
    media_types: &["application/vnd.kde.kchart"],
    signatures: &[],
    related_formats: &[],
};
