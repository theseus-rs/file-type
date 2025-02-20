use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967107: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_107,
        source_type: SourceType::Wikidata,
        name: "Ragnarok Online 2 RMP",
        extensions: &["rmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
