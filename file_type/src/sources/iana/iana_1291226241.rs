use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1291226241: FileType = FileType {
    file_format: &FileFormat {
        id: 1_291_226_241,
        source_type: SourceType::Iana,
        name: "vnd.onepager",
        extensions: &[],
        media_types: &["application/vnd.onepager"],
        signatures: &[],
        related_formats: &[],
    },
};
