use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979154: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_154,
        source_type: SourceType::Wikidata,
        name: "ArtWorx Data Format",
        extensions: &["adf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
