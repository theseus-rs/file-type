use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28757998: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_998,
        source_type: SourceType::Wikidata,
        name: "Inflate",
        extensions: &["infl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
