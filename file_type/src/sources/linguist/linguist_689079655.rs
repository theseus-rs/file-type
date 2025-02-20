use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_689079655: FileType = FileType {
    file_format: &FileFormat {
        id: 689_079_655,
        source_type: SourceType::Linguist,
        name: "OverpassQL",
        extensions: &["overpassql"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
