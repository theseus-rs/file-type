use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_372626: FileType = FileType {
    file_format: &FileFormat {
        id: 372_626,
        source_type: SourceType::Wikidata,
        name: "Theora",
        extensions: &["ogg", "ogv"],
        media_types: &["video/theora"],
        signatures: &[],
        related_formats: &[],
    },
};
