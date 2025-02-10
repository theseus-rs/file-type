use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975737: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_737,
        source_type: SourceType::Wikidata,
        name: "POV-Ray density file",
        extensions: &["df3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
