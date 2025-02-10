use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3644124055: FileType = FileType {
    file_format: &FileFormat {
        id: 3_644_124_055,
        source_type: SourceType::Iana,
        name: "vnd.fujixerox.ART4",
        extensions: &[],
        media_types: &["application/vnd.fujixerox.ART4"],
        signatures: &[],
        related_formats: &[],
    },
};
