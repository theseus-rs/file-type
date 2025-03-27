use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967512: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_512,
        source_type: SourceType::Wikidata,
        name: "Matroska Video",
        extensions: &["mkv"],
        media_types: &["video/matroska", "video/x-matroska"],
        signatures: &[],
        related_formats: &[],
    },
};
