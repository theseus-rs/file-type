use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_10397009: FileType = FileType {
    file_format: &FileFormat {
        id: 10_397_009,
        source_type: SourceType::Wikidata,
        name: "Arachne Plugin Manager file format",
        extensions: &["apm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
