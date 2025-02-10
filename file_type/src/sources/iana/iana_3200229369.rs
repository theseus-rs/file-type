use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3200229369: FileType = FileType {
    file_format: &FileFormat {
        id: 3_200_229_369,
        source_type: SourceType::Iana,
        name: "flexfec",
        extensions: &[],
        media_types: &["text/flexfec"],
        signatures: &[],
        related_formats: &[],
    },
};
