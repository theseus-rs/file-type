use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1821710676: FileType = FileType {
    file_format: &FileFormat {
        id: 1_821_710_676,
        source_type: SourceType::Iana,
        name: "vnd.g3pix.g3fc",
        extensions: &[],
        media_types: &["application/vnd.g3pix.g3fc"],
        signatures: &[],
        related_formats: &[],
    },
};
