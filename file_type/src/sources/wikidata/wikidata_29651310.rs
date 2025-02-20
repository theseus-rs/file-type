use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29651310: FileType = FileType {
    file_format: &FileFormat {
        id: 29_651_310,
        source_type: SourceType::Wikidata,
        name: "Pixie",
        extensions: &["pxi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
