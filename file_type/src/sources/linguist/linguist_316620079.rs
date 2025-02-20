use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_316620079: FileType = FileType {
    file_format: &FileFormat {
        id: 316_620_079,
        source_type: SourceType::Linguist,
        name: "JCL",
        extensions: &["jcl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
