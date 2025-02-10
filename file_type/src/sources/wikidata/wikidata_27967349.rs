use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967349: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_349,
        source_type: SourceType::Wikidata,
        name: "iTunes Music Library, binary variant",
        extensions: &["itl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
