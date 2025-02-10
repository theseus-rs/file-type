use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3910194067: FileType = FileType {
    file_format: &FileFormat {
        id: 3_910_194_067,
        source_type: SourceType::Iana,
        name: "vnd.fastcopy-disk-image",
        extensions: &[],
        media_types: &["application/vnd.fastcopy-disk-image"],
        signatures: &[],
        related_formats: &[],
    },
};
