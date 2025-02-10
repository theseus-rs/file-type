use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967518: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_518,
        source_type: SourceType::Wikidata,
        name: "Matroska Subtitles",
        extensions: &["mks"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
