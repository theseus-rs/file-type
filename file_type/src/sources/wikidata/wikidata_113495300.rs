use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113495300: FileType = FileType {
    file_format: &FileFormat {
        id: 113_495_300,
        source_type: SourceType::Wikidata,
        name: "JPEG XS File Format",
        extensions: &["jxs"],
        media_types: &["image/jxs"],
        signatures: &[],
        related_formats: &[],
    },
};
