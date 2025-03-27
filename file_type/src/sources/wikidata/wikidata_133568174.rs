use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133568174: FileType = FileType {
    file_format: &FileFormat {
        id: 133_568_174,
        source_type: SourceType::Wikidata,
        name: "DUO file",
        extensions: &["du1", "du2", "duo"],
        media_types: &["image/x-atari-duo"],
        signatures: &[],
        related_formats: &[],
    },
};
