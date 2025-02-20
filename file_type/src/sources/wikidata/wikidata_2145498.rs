use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2145498: FileType = FileType {
    file_format: &FileFormat {
        id: 2_145_498,
        source_type: SourceType::Wikidata,
        name: "Requirements Interchange Format",
        extensions: &["reqif", "reqifz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
