use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_545120659: FileFormat = FileFormat {
    id: 545_120_659,
    source_type: SourceType::Iana,
    name: "vnd.dts",
    extensions: &[],
    media_types: &["audio/vnd.dts"],
    signatures: &[],
    related_formats: &[],
};
