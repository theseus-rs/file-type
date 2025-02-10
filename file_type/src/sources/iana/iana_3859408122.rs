use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3859408122: FileType = FileType {
    file_format: &FileFormat {
        id: 3_859_408_122,
        source_type: SourceType::Iana,
        name: "vnd.cups-postscript",
        extensions: &[],
        media_types: &["application/vnd.cups-postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
