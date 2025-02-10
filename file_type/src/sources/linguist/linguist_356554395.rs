use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_356554395: FileType = FileType {
    file_format: &FileFormat {
        id: 356_554_395,
        source_type: SourceType::Linguist,
        name: "Toit",
        extensions: &["toit"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
