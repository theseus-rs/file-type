use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133869876: FileType = FileType {
    file_format: &FileFormat {
        id: 133_869_876,
        source_type: SourceType::Wikidata,
        name: "Telepaint file",
        extensions: &["ss", "st"],
        media_types: &["image/x-telepaint"],
        signatures: &[],
        related_formats: &[],
    },
};
