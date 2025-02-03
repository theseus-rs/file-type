use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2905229348: FileFormat = FileFormat {
    id: 2_905_229_348,
    source_type: SourceType::Iana,
    name: "vnd.oasis.opendocument.chart",
    extensions: &[],
    media_types: &["application/vnd.oasis.opendocument.chart"],
    internal_signatures: &[],
    related_formats: &[],
};
