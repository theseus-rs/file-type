use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3327694988: FileFormat = FileFormat {
    id: 3_327_694_988,
    source_type: SourceType::Iana,
    name: "vnd.sealed.swf",
    extensions: &[],
    media_types: &["video/vnd.sealed.swf"],
    signatures: &[],
    related_formats: &[],
};
