use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127701093: FileType = FileType {
    file_format: &FileFormat {
        id: 127_701_093,
        source_type: SourceType::Wikidata,
        name: "Hack source code file",
        extensions: &["hack"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
