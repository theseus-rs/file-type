use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110098625: FileType = FileType {
    file_format: &FileFormat {
        id: 110_098_625,
        source_type: SourceType::Wikidata,
        name: "Exif Image File Format (Compressed)",
        extensions: &["jpeg", "jpg"],
        media_types: &["image/jpeg"],
        signatures: &[],
        related_formats: &[],
    },
};
