use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3449603082: FileFormat = FileFormat {
    id: 3_449_603_082,
    source_type: SourceType::Iana,
    name: "vnd.fujixerox.ART-EX",
    extensions: &[],
    media_types: &["application/vnd.fujixerox.ART-EX"],
    signatures: &[],
    related_formats: &[],
};
