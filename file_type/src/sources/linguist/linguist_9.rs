use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_9: FileType = FileType {
    file_format: &FileFormat {
        id: 9,
        source_type: SourceType::Linguist,
        name: "ATS",
        extensions: &["dats", "hats", "sats"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
