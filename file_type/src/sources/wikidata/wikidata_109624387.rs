use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109624387: FileType = FileType {
    file_format: &FileFormat {
        id: 109_624_387,
        source_type: SourceType::Wikidata,
        name: "WebPlus Essentials Templates",
        extensions: &["wpx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
