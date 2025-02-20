use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_900799277: FileType = FileType {
    file_format: &FileFormat {
        id: 900_799_277,
        source_type: SourceType::Iana,
        name: "vnd.nokia.iSDS-radio-presets",
        extensions: &[],
        media_types: &["application/vnd.nokia.iSDS-radio-presets"],
        signatures: &[],
        related_formats: &[],
    },
};
