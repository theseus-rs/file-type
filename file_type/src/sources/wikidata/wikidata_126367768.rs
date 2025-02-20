use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126367768: FileType = FileType {
    file_format: &FileFormat {
        id: 126_367_768,
        source_type: SourceType::Wikidata,
        name: "KPhotoAlbum file",
        extensions: &["kim"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
