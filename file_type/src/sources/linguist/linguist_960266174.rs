use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_960266174: FileType = FileType {
    file_format: &FileFormat {
        id: 960_266_174,
        source_type: SourceType::Linguist,
        name: "Starlark",
        extensions: &["bzl", "star"],
        media_types: &["text/x-python"],
        signatures: &[],
        related_formats: &[],
    },
};
