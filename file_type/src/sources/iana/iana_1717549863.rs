use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1717549863: FileFormat = FileFormat {
    id: 1_717_549_863,
    source_type: SourceType::Iana,
    name: "vnd.dolby.pulse.1",
    extensions: &[],
    media_types: &["audio/vnd.dolby.pulse.1"],
    signatures: &[],
    related_formats: &[],
};
