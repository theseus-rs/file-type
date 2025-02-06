use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1926351755: FileFormat = FileFormat {
    id: 1_926_351_755,
    source_type: SourceType::Iana,
    name: "H263-1998",
    extensions: &[],
    media_types: &["video/H263-1998"],
    signatures: &[],
    related_formats: &[],
};
