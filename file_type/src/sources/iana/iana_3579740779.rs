use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3579740779: FileFormat = FileFormat {
    id: 3_579_740_779,
    source_type: SourceType::Iana,
    name: "vnd.dir-bi.plate-dl-nosuffix",
    extensions: &[],
    media_types: &["application/vnd.dir-bi.plate-dl-nosuffix"],
    signatures: &[],
    related_formats: &[],
};
