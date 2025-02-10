use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_93: FileType = FileType {
    file_format: &FileFormat {
        id: 93,
        source_type: SourceType::Linguist,
        name: "ECL",
        extensions: &["ecl", "eclxml"],
        media_types: &["text/x-ecl"],
        signatures: &[],
        related_formats: &[],
    },
};
