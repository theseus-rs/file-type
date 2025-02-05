use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_241505788: FileFormat = FileFormat {
    id: 241_505_788,
    source_type: SourceType::Iana,
    name: "vnd.nortel.vbk",
    extensions: &[],
    media_types: &["audio/vnd.nortel.vbk"],
    signatures: &[],
    related_formats: &[],
};
