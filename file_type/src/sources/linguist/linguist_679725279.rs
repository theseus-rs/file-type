use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_679725279: FileType = FileType {
    file_format: &FileFormat {
        id: 679_725_279,
        source_type: SourceType::Linguist,
        name: "HOCON",
        extensions: &["hocon"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
