use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3311782725: FileType = FileType {
    file_format: &FileFormat {
        id: 3_311_782_725,
        source_type: SourceType::Iana,
        name: "vnd.DMClientScript",
        extensions: &[],
        media_types: &["text/vnd.DMClientScript"],
        signatures: &[],
        related_formats: &[],
    },
};
