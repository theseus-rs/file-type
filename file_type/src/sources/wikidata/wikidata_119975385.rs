use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119975385: FileType = FileType {
    file_format: &FileFormat {
        id: 119_975_385,
        source_type: SourceType::Wikidata,
        name: "Style Template",
        extensions: &["tps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
