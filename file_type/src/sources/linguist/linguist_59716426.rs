use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_59716426: FileType = FileType {
    file_format: &FileFormat {
        id: 59_716_426,
        source_type: SourceType::Linguist,
        name: "KerboScript",
        extensions: &["ks"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
