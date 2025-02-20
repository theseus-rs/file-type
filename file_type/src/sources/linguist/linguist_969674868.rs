use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_969674868: FileType = FileType {
    file_format: &FileFormat {
        id: 969_674_868,
        source_type: SourceType::Linguist,
        name: "Windows Registry Entries",
        extensions: &["reg"],
        media_types: &["text/x-properties"],
        signatures: &[],
        related_formats: &[],
    },
};
