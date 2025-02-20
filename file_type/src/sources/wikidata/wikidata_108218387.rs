use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_108218387: FileType = FileType {
    file_format: &FileFormat {
        id: 108_218_387,
        source_type: SourceType::Wikidata,
        name: "Citation File Format",
        extensions: &["cff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
