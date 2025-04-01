use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133576410: FileType = FileType {
    file_format: &FileFormat {
        id: 133_576_410,
        source_type: SourceType::Wikidata,
        name: "Hi-Eddi file",
        extensions: &["hed"],
        media_types: &["image/x-hi-eddi"],
        signatures: &[],
        related_formats: &[],
    },
};
