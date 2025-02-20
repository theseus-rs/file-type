use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27936287: FileType = FileType {
    file_format: &FileFormat {
        id: 27_936_287,
        source_type: SourceType::Wikidata,
        name: "Earth Resources Laboratory Applications Software",
        extensions: &["elas"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
