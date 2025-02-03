use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3076879620: FileFormat = FileFormat {
    id: 3_076_879_620,
    source_type: SourceType::Iana,
    name: "vnd.kde.kword",
    extensions: &[],
    media_types: &["application/vnd.kde.kword"],
    internal_signatures: &[],
    related_formats: &[],
};
