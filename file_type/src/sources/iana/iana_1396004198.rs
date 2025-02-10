use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1396004198: FileType = FileType {
    file_format: &FileFormat {
        id: 1_396_004_198,
        source_type: SourceType::Iana,
        name: "vnd.modl",
        extensions: &[],
        media_types: &["application/vnd.modl"],
        signatures: &[],
        related_formats: &[],
    },
};
