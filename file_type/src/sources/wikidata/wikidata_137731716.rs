use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137731716: FileType = FileType {
    file_format: &FileFormat {
        id: 137_731_716,
        source_type: SourceType::Wikidata,
        name: "JavaScript Syntax Extension file",
        extensions: &["jsx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
