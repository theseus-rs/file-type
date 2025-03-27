use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_18415256: FileType = FileType {
    file_format: &FileFormat {
        id: 18_415_256,
        source_type: SourceType::Wikidata,
        name: "Scalable Vector Graphics Font",
        extensions: &["svg"],
        media_types: &["image/svg+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
