use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_11222362: FileType = FileType {
    file_format: &FileFormat {
        id: 11_222_362,
        source_type: SourceType::Wikidata,
        name: "HD Photo",
        extensions: &["wdp"],
        media_types: &["image/vnd.ms-photo"],
        signatures: &[],
        related_formats: &[],
    },
};
