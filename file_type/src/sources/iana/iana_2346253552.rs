use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2346253552: FileType = FileType {
    file_format: &FileFormat {
        id: 2_346_253_552,
        source_type: SourceType::Iana,
        name: "vnd.comicbook-rar",
        extensions: &[],
        media_types: &["application/vnd.comicbook-rar"],
        signatures: &[],
        related_formats: &[],
    },
};
