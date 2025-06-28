use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_694638086: FileType = FileType {
    file_format: &FileFormat {
        id: 694_638_086,
        source_type: SourceType::Linguist,
        name: "GDShader",
        extensions: &["gdshader", "gdshaderinc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
