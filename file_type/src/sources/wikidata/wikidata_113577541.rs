use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113577541: FileType = FileType {
    file_format: &FileFormat {
        id: 113_577_541,
        source_type: SourceType::Wikidata,
        name: "DiscJuggler Image",
        extensions: &["cdi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
