use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1016912802: FileType = FileType {
    file_format: &FileFormat {
        id: 1_016_912_802,
        source_type: SourceType::Linguist,
        name: "Tor Config",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
