use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131748260: FileType = FileType {
    file_format: &FileFormat {
        id: 131_748_260,
        source_type: SourceType::Wikidata,
        name: "Parallel Input Output file",
        extensions: &["pio"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
