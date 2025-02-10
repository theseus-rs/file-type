use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_716513858: FileType = FileType {
    file_format: &FileFormat {
        id: 716_513_858,
        source_type: SourceType::Linguist,
        name: "Proguard",
        extensions: &["pro"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
