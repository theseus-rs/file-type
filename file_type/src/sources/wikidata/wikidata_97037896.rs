use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_97037896: FileType = FileType {
    file_format: &FileFormat {
        id: 97_037_896,
        source_type: SourceType::Wikidata,
        name: "Personal Icon",
        extensions: &["picon"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
