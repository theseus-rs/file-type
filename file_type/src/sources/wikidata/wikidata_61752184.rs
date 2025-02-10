use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61752184: FileType = FileType {
    file_format: &FileFormat {
        id: 61_752_184,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version CS",
        extensions: &["ind", "indd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
