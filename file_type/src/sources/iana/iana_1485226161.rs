use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1485226161: FileType = FileType {
    file_format: &FileFormat {
        id: 1_485_226_161,
        source_type: SourceType::Iana,
        name: "vnd.collabio.xodocuments.document",
        extensions: &[],
        media_types: &["application/vnd.collabio.xodocuments.document"],
        signatures: &[],
        related_formats: &[],
    },
};
