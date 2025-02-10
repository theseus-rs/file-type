use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_1045019587: FileType = FileType {
    file_format: &FileFormat {
        id: 1_045_019_587,
        source_type: SourceType::Linguist,
        name: "Mojo",
        extensions: &["mojo"],
        media_types: &["text/x-python"],
        signatures: &[],
        related_formats: &[],
    },
};
