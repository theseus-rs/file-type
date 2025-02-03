use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2437831108: FileFormat = FileFormat {
    id: 2_437_831_108,
    source_type: SourceType::Iana,
    name: "vnd.kde.karbon",
    extensions: &[],
    media_types: &["application/vnd.kde.karbon"],
    internal_signatures: &[],
    related_formats: &[],
};
