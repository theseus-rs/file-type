use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_433224: FileType = FileType {
    file_format: &FileFormat {
        id: 433_224,
        source_type: SourceType::Wikidata,
        name: "APNG",
        extensions: &["apng", "png"],
        media_types: &["image/apng", "image/png"],
        signatures: &[],
        related_formats: &[],
    },
};
