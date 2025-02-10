use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3917093790: FileType = FileType {
    file_format: &FileFormat {
        id: 3_917_093_790,
        source_type: SourceType::Iana,
        name: "vnd.rosette.annotated-data-model",
        extensions: &[],
        media_types: &["model/vnd.rosette.annotated-data-model"],
        signatures: &[],
        related_formats: &[],
    },
};
