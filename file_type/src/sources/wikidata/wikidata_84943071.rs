use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_84943071: FileType = FileType {
    file_format: &FileFormat {
        id: 84_943_071,
        source_type: SourceType::Wikidata,
        name: "Sony PictureGear Studio Binder",
        extensions: &["bxt", "bxu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
