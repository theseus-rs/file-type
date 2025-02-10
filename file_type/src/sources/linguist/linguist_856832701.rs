use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_856832701: FileType = FileType {
    file_format: &FileFormat {
        id: 856_832_701,
        source_type: SourceType::Linguist,
        name: "Terraform Template",
        extensions: &["tftpl"],
        media_types: &["text/x-ruby"],
        signatures: &[],
        related_formats: &[],
    },
};
