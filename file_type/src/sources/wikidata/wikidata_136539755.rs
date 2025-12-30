use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136539755: FileType = FileType {
    file_format: &FileFormat {
        id: 136_539_755,
        source_type: SourceType::Wikidata,
        name: "GRAPHITE File",
        extensions: &["GRAPHITE"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
