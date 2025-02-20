use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2640148687: FileType = FileType {
    file_format: &FileFormat {
        id: 2_640_148_687,
        source_type: SourceType::Iana,
        name: "vnd.filmit.zfc",
        extensions: &[],
        media_types: &["application/vnd.filmit.zfc"],
        signatures: &[],
        related_formats: &[],
    },
};
