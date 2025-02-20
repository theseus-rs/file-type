use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206593: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_593,
        source_type: SourceType::Wikidata,
        name: "MSX Interchange Format",
        extensions: &["mif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
