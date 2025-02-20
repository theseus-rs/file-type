use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_692040152: FileType = FileType {
    file_format: &FileFormat {
        id: 692_040_152,
        source_type: SourceType::Iana,
        name: "vnd.svf",
        extensions: &[],
        media_types: &["image/vnd.svf"],
        signatures: &[],
        related_formats: &[],
    },
};
