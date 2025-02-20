use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205807: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_807,
        source_type: SourceType::Wikidata,
        name: "Palette Format",
        extensions: &["pal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
