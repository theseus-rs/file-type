use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_84616920: FileType = FileType {
    file_format: &FileFormat {
        id: 84_616_920,
        source_type: SourceType::Iana,
        name: "EVRCB",
        extensions: &[],
        media_types: &["audio/EVRCB"],
        signatures: &[],
        related_formats: &[],
    },
};
