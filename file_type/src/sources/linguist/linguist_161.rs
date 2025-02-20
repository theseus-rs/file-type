use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_161: FileType = FileType {
    file_format: &FileFormat {
        id: 161,
        source_type: SourceType::Linguist,
        name: "IDL",
        extensions: &["dlm", "pro"],
        media_types: &["text/x-idl"],
        signatures: &[],
        related_formats: &[],
    },
};
