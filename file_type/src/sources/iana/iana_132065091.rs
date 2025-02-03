use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_132065091: FileFormat = FileFormat {
    id: 132_065_091,
    source_type: SourceType::Iana,
    name: "iso.segment",
    extensions: &[],
    media_types: &["video/iso.segment"],
    internal_signatures: &[],
    related_formats: &[],
};
