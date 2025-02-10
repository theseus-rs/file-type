use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_374: FileType = FileType {
    file_format: &FileFormat {
        id: 374,
        source_type: SourceType::Linguist,
        name: "Thrift",
        extensions: &["thrift"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
