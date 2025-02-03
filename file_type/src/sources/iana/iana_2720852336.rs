use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2720852336: FileFormat = FileFormat {
    id: 2_720_852_336,
    source_type: SourceType::Iana,
    name: "vnd.radgamettools.bink",
    extensions: &[],
    media_types: &["video/vnd.radgamettools.bink"],
    internal_signatures: &[],
    related_formats: &[],
};
