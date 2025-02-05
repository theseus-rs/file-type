use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1771736547: FileFormat = FileFormat {
    id: 1_771_736_547,
    source_type: SourceType::Iana,
    name: "vnd.bbf.usp.error",
    extensions: &[],
    media_types: &["application/vnd.bbf.usp.error"],
    signatures: &[],
    related_formats: &[],
};
