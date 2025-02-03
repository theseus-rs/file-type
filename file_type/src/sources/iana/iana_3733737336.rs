use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3733737336: FileFormat = FileFormat {
    id: 3_733_737_336,
    source_type: SourceType::Iana,
    name: "H263-2000",
    extensions: &[],
    media_types: &["video/H263-2000"],
    internal_signatures: &[],
    related_formats: &[],
};
