use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2620488527: FileType = FileType {
    file_format: &FileFormat {
        id: 2_620_488_527,
        source_type: SourceType::Iana,
        name: "trickle-ice-sdpfrag",
        extensions: &[],
        media_types: &["application/trickle-ice-sdpfrag"],
        signatures: &[],
        related_formats: &[],
    },
};
