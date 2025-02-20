use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757900: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_900,
        source_type: SourceType::Wikidata,
        name: "Glyph Interchange Format",
        extensions: &["glif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
