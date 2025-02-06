use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4236266180: FileFormat = FileFormat {
    id: 4_236_266_180,
    source_type: SourceType::Iana,
    name: "mls",
    extensions: &[],
    media_types: &["message/mls"],
    signatures: &[],
    related_formats: &[],
};
