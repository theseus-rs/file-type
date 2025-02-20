use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_263: FileType = FileType {
    file_format: &FileFormat {
        id: 263,
        source_type: SourceType::Linguist,
        name: "OpenCL",
        extensions: &["cl", "opencl"],
        media_types: &["text/x-csrc"],
        signatures: &[],
        related_formats: &[],
    },
};
