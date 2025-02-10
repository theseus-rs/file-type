use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4293215786: FileType = FileType {
    file_format: &FileFormat {
        id: 4_293_215_786,
        source_type: SourceType::Iana,
        name: "vnd.acucobol",
        extensions: &[],
        media_types: &["application/vnd.acucobol"],
        signatures: &[],
        related_formats: &[],
    },
};
