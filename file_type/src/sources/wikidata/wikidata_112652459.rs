use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112652459: FileType = FileType {
    file_format: &FileFormat {
        id: 112_652_459,
        source_type: SourceType::Wikidata,
        name: "Astound file format",
        extensions: &["asd"],
        media_types: &["x-application/Astound"],
        signatures: &[],
        related_formats: &[],
    },
};
