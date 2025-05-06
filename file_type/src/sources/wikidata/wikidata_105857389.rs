use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105857389: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_389,
        source_type: SourceType::Wikidata,
        name: "JSON Entity Model (Minecraft OptiFine)",
        extensions: &["jem"],
        media_types: &["text/json"],
        signatures: &[],
        related_formats: &[],
    },
};
