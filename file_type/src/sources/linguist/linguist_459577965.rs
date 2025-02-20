use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_459577965: FileType = FileType {
    file_format: &FileFormat {
        id: 459_577_965,
        source_type: SourceType::Linguist,
        name: "GEDCOM",
        extensions: &["ged"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
