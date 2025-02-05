use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_428848062: FileFormat = FileFormat {
    id: 428_848_062,
    source_type: SourceType::Iana,
    name: "vnd.4SB",
    extensions: &[],
    media_types: &["audio/vnd.4SB"],
    signatures: &[],
    related_formats: &[],
};
