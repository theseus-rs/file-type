use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132539943: FileType = FileType {
    file_format: &FileFormat {
        id: 132_539_943,
        source_type: SourceType::Wikidata,
        name: "Surface Domain Properties file format",
        extensions: &["oprops"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
