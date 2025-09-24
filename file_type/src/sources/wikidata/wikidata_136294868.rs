use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136294868: FileType = FileType {
    file_format: &FileFormat {
        id: 136_294_868,
        source_type: SourceType::Wikidata,
        name: "Notion file format",
        extensions: &["notion"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
