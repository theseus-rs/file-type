use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2996336798: FileFormat = FileFormat {
    id: 2_996_336_798,
    source_type: SourceType::Iana,
    name: "vnd.lotus-screencam",
    extensions: &[],
    media_types: &["application/vnd.lotus-screencam"],
    internal_signatures: &[],
    related_formats: &[],
};
