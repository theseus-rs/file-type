use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_95994246: FileType = FileType {
    file_format: &FileFormat {
        id: 95_994_246,
        source_type: SourceType::Wikidata,
        name: "Agilent Microarray file format",
        extensions: &["txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
