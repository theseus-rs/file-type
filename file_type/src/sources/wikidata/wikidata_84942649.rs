use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_84942649: FileType = FileType {
    file_format: &FileFormat {
        id: 84_942_649,
        source_type: SourceType::Wikidata,
        name: "Sony PictureGear Studio PhotoAlbum",
        extensions: &["amd", "amu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
