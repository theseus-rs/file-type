use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136841330: FileType = FileType {
    file_format: &FileFormat {
        id: 136_841_330,
        source_type: SourceType::Wikidata,
        name: "Heredis file format",
        extensions: &["hmw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
