use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4192850660: FileType = FileType {
    file_format: &FileFormat {
        id: 4_192_850_660,
        source_type: SourceType::Iana,
        name: "rtp-midi",
        extensions: &[],
        media_types: &["audio/rtp-midi"],
        signatures: &[],
        related_formats: &[],
    },
};
