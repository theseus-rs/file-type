use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_802613624: FileType = FileType {
    file_format: &FileFormat {
        id: 802_613_624,
        source_type: SourceType::Iana,
        name: "EVRCWB",
        extensions: &[],
        media_types: &["audio/EVRCWB"],
        signatures: &[],
        related_formats: &[],
    },
};
