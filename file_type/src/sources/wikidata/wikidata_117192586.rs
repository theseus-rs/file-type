use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117192586: FileType = FileType {
    file_format: &FileFormat {
        id: 117_192_586,
        source_type: SourceType::Wikidata,
        name: "Generic Encapsulated PostScript Graphic Format",
        extensions: &["ai3", "ai4", "ai5", "ai6", "ai7", "ai8"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
