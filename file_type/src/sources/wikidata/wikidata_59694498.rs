use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59694498: FileType = FileType {
    file_format: &FileFormat {
        id: 59_694_498,
        source_type: SourceType::Wikidata,
        name: "i2 Analysts Notebook",
        extensions: &["anb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
