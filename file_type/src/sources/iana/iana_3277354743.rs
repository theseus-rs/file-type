use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3277354743: FileFormat = FileFormat {
    id: 3_277_354_743,
    source_type: SourceType::Iana,
    name: "vnd.apple.keynote",
    extensions: &[],
    media_types: &["application/vnd.apple.keynote"],
    internal_signatures: &[],
    related_formats: &[],
};
