use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_745244408: FileFormat = FileFormat {
    id: 745_244_408,
    source_type: SourceType::Iana,
    name: "vnd.aether.imp",
    extensions: &[],
    media_types: &["application/vnd.aether.imp"],
    signatures: &[],
    related_formats: &[],
};
