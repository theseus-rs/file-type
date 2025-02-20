use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27231651: FileType = FileType {
    file_format: &FileFormat {
        id: 27_231_651,
        source_type: SourceType::Wikidata,
        name: "Tag Image File Format, version 5.0",
        extensions: &["tif"],
        media_types: &["image/tiff"],
        signatures: &[],
        related_formats: &[],
    },
};
