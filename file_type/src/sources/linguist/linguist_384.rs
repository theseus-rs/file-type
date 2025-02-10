use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_384: FileType = FileType {
    file_format: &FileFormat {
        id: 384,
        source_type: SourceType::Linguist,
        name: "VCL",
        extensions: &["vcl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
