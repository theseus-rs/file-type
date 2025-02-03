use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2394236616: FileFormat = FileFormat {
    id: 2_394_236_616,
    source_type: SourceType::Iana,
    name: "vnd.sealed.ppt",
    extensions: &[],
    media_types: &["application/vnd.sealed.ppt"],
    internal_signatures: &[],
    related_formats: &[],
};
