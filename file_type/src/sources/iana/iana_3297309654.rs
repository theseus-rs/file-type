use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3297309654: FileType = FileType {
    file_format: &FileFormat {
        id: 3_297_309_654,
        source_type: SourceType::Iana,
        name: "G729D",
        extensions: &[],
        media_types: &["audio/G729D"],
        signatures: &[],
        related_formats: &[],
    },
};
