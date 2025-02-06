use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3037433719: FileFormat = FileFormat {
    id: 3_037_433_719,
    source_type: SourceType::Iana,
    name: "vnd.gerber",
    extensions: &[],
    media_types: &["application/vnd.gerber"],
    signatures: &[],
    related_formats: &[],
};
