use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3683296436: FileFormat = FileFormat {
    id: 3_683_296_436,
    source_type: SourceType::Iana,
    name: "vnd.adobe.formscentral.fcdt",
    extensions: &[],
    media_types: &["application/vnd.adobe.formscentral.fcdt"],
    signatures: &[],
    related_formats: &[],
};
