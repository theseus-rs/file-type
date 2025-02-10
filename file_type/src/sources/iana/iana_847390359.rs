use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_847390359: FileType = FileType {
    file_format: &FileFormat {
        id: 847_390_359,
        source_type: SourceType::Iana,
        name: "vnd.easykaraoke.cdgdownload",
        extensions: &[],
        media_types: &["application/vnd.easykaraoke.cdgdownload"],
        signatures: &[],
        related_formats: &[],
    },
};
