use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_390788699: FileType = FileType {
    file_format: &FileFormat {
        id: 390_788_699,
        source_type: SourceType::Linguist,
        name: "CAP CDS",
        extensions: &["cds"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
