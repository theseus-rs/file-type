use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
