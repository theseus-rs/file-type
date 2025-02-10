use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3981745143: FileType = FileType {
    file_format: &FileFormat {
        id: 3_981_745_143,
        source_type: SourceType::Iana,
        name: "vnd.webturbo",
        extensions: &[],
        media_types: &["application/vnd.webturbo"],
        signatures: &[],
        related_formats: &[],
    },
};
