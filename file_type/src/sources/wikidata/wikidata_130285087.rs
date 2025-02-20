use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130285087: FileType = FileType {
    file_format: &FileFormat {
        id: 130_285_087,
        source_type: SourceType::Wikidata,
        name: "Minecraft Add-ons Data Schema File",
        extensions: &["mcschema"],
        media_types: &["text/mcschema"],
        signatures: &[],
        related_formats: &[],
    },
};
