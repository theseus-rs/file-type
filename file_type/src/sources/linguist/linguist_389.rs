use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_389: FileType = FileType {
    file_format: &FileFormat {
        id: 389,
        source_type: SourceType::Linguist,
        name: "Visual Basic .NET",
        extensions: &["vb", "vbhtml"],
        media_types: &["text/x-vb"],
        signatures: &[],
        related_formats: &[],
    },
};
