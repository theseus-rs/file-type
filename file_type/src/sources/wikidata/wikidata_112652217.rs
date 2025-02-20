use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112652217: FileType = FileType {
    file_format: &FileFormat {
        id: 112_652_217,
        source_type: SourceType::Wikidata,
        name: "VIZ Render file format",
        extensions: &["drf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
