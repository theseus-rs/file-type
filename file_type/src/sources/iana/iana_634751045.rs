use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_634751045: FileFormat = FileFormat {
    id: 634_751_045,
    source_type: SourceType::Iana,
    name: "vnd.cosmocaller",
    extensions: &[],
    media_types: &["application/vnd.cosmocaller"],
    internal_signatures: &[],
    related_formats: &[],
};
