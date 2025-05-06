use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6731061: FileType = FileType {
    file_format: &FileFormat {
        id: 6_731_061,
        source_type: SourceType::Wikidata,
        name: "Magick Image File Format",
        extensions: &["mif", "miff"],
        media_types: &["image/miff", "image/x-miff"],
        signatures: &[],
        related_formats: &[],
    },
};
