use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1222670581: FileFormat = FileFormat {
    id: 1_222_670_581,
    source_type: SourceType::Iana,
    name: "vnd.dece.sd",
    extensions: &[],
    media_types: &["video/vnd.dece.sd"],
    signatures: &[],
    related_formats: &[],
};
