use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1023933872: FileFormat = FileFormat {
    id: 1_023_933_872,
    source_type: SourceType::Iana,
    name: "vnd.dolby.pl2",
    extensions: &[],
    media_types: &["audio/vnd.dolby.pl2"],
    internal_signatures: &[],
    related_formats: &[],
};
