use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_56827134: FileType = FileType {
    file_format: &FileFormat {
        id: 56_827_134,
        source_type: SourceType::Wikidata,
        name: "PicoTech Picologger PLW",
        extensions: &["plw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
