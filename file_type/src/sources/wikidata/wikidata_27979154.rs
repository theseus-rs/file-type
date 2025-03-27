use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979154: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_154,
        source_type: SourceType::Wikidata,
        name: "ArtWorx Data Format",
        extensions: &["adf"],
        media_types: &["image/x-artworx"],
        signatures: &[],
        related_formats: &[],
    },
};
