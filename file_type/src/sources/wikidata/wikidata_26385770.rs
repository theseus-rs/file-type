use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_26385770: FileType = FileType {
    file_format: &FileFormat {
        id: 26_385_770,
        source_type: SourceType::Wikidata,
        name: "Still Picture Interchange File Format",
        extensions: &["jpeg", "jpg", "spf", "spiff"],
        media_types: &["image/jpeg"],
        signatures: &[],
        related_formats: &[],
    },
};
