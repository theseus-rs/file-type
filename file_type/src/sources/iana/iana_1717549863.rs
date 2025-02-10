use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1717549863: FileType = FileType {
    file_format: &FileFormat {
        id: 1_717_549_863,
        source_type: SourceType::Iana,
        name: "vnd.dolby.pulse.1",
        extensions: &[],
        media_types: &["audio/vnd.dolby.pulse.1"],
        signatures: &[],
        related_formats: &[],
    },
};
