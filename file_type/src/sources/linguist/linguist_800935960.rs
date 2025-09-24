use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_800935960: FileType = FileType {
    file_format: &FileFormat {
        id: 800_935_960,
        source_type: SourceType::Linguist,
        name: "Flix",
        extensions: &["flix"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
