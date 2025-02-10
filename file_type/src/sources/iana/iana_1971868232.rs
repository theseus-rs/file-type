use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1971868232: FileType = FileType {
    file_format: &FileFormat {
        id: 1_971_868_232,
        source_type: SourceType::Iana,
        name: "mhas",
        extensions: &[],
        media_types: &["audio/mhas"],
        signatures: &[],
        related_formats: &[],
    },
};
