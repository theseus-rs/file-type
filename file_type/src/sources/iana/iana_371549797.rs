use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_371549797: FileFormat = FileFormat {
    id: 371_549_797,
    source_type: SourceType::Iana,
    name: "vnd.dece.video",
    extensions: &[],
    media_types: &["video/vnd.dece.video"],
    internal_signatures: &[],
    related_formats: &[],
};
