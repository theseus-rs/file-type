use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_664257356: FileType = FileType {
    file_format: &FileFormat {
        id: 664_257_356,
        source_type: SourceType::Linguist,
        name: "ShaderLab",
        extensions: &["shader"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
