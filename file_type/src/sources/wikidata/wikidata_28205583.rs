use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205583: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_583,
        source_type: SourceType::Wikidata,
        name: "OS/2 Pointer",
        extensions: &["ptr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
