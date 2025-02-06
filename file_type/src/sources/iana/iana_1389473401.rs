use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1389473401: FileFormat = FileFormat {
    id: 1_389_473_401,
    source_type: SourceType::Iana,
    name: "vnd.kde.kpresenter",
    extensions: &[],
    media_types: &["application/vnd.kde.kpresenter"],
    signatures: &[],
    related_formats: &[],
};
