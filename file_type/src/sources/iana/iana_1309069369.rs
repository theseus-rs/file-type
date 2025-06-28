use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1309069369: FileType = FileType {
    file_format: &FileFormat {
        id: 1_309_069_369,
        source_type: SourceType::Iana,
        name: "vnd.fgb",
        extensions: &[],
        media_types: &["application/vnd.fgb"],
        signatures: &[],
        related_formats: &[],
    },
};
