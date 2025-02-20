use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2978006526: FileType = FileType {
    file_format: &FileFormat {
        id: 2_978_006_526,
        source_type: SourceType::Iana,
        name: "rtx",
        extensions: &[],
        media_types: &["text/rtx"],
        signatures: &[],
        related_formats: &[],
    },
};
