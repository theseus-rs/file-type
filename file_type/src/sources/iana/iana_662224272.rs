use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_662224272: FileType = FileType {
    file_format: &FileFormat {
        id: 662_224_272,
        source_type: SourceType::Iana,
        name: "step",
        extensions: &[],
        media_types: &["model/step"],
        signatures: &[],
        related_formats: &[],
    },
};
