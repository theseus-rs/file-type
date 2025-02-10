use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4235591232: FileType = FileType {
    file_format: &FileFormat {
        id: 4_235_591_232,
        source_type: SourceType::Iana,
        name: "vnd.eudora.data",
        extensions: &[],
        media_types: &["application/vnd.eudora.data"],
        signatures: &[],
        related_formats: &[],
    },
};
