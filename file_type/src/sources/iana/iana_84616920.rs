use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
