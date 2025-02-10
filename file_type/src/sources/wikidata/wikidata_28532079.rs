use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28532079: FileType = FileType {
    file_format: &FileFormat {
        id: 28_532_079,
        source_type: SourceType::Wikidata,
        name: "Alchemy Format",
        extensions: &["alc"],
        media_types: &["chemical/x-alchemy"],
        signatures: &[],
        related_formats: &[],
    },
};
