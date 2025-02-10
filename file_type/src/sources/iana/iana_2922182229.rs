use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2922182229: FileType = FileType {
    file_format: &FileFormat {
        id: 2_922_182_229,
        source_type: SourceType::Iana,
        name: "vnd.dlna.mpeg-tts",
        extensions: &[],
        media_types: &["video/vnd.dlna.mpeg-tts"],
        signatures: &[],
        related_formats: &[],
    },
};
