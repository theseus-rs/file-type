use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_425: FileType = FileType {
    file_format: &FileFormat {
        id: 425,
        source_type: SourceType::Linguist,
        name: "Pic",
        extensions: &["chem", "pic"],
        media_types: &["text/troff"],
        signatures: &[],
        related_formats: &[],
    },
};
