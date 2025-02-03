use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2723193037: FileFormat = FileFormat {
    id: 2_723_193_037,
    source_type: SourceType::Iana,
    name: "vnd.kde.kformula",
    extensions: &[],
    media_types: &["application/vnd.kde.kformula"],
    internal_signatures: &[],
    related_formats: &[],
};
